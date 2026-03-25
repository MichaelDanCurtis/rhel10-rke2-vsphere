#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Build RHEL 10 Image Mode — RKE2 + vSphere ISO
# ============================================================
# Prerequisites:
#   - podman installed (rootful on macOS: podman machine set --rootful)
#   - Authenticated to registry.redhat.io: podman login registry.redhat.io
#   - subscription-manager registered inside podman machine
# ============================================================

IMAGE_NAME="localhost/rhel10-rke2-vsphere"
IMAGE_TAG="latest"
OUTPUT_DIR="$(pwd)/output"
VM_BUILD_DIR="/tmp/rhel-build"

# ---- Step 0: Copy build context into the podman machine VM ----
echo ">>> Copying build context to podman machine..."
podman machine ssh -- "rm -rf ${VM_BUILD_DIR} && mkdir -p ${VM_BUILD_DIR}/files/entitlement"

# Copy files into the VM
for f in Containerfile config.toml; do
  cat "$f" | podman machine ssh -- "cat > ${VM_BUILD_DIR}/$f"
done
for f in files/*.repo files/*.yaml files/*.toml files/*.conf; do
  [ -f "$f" ] && cat "$f" | podman machine ssh -- "cat > ${VM_BUILD_DIR}/$f"
done
for f in files/entitlement/*; do
  [ -f "$f" ] && cat "$f" | podman machine ssh -- "cat > ${VM_BUILD_DIR}/$f"
done

# ---- Step 1: Build the bootc container image inside the VM ----
echo ">>> Building bootc container image..."
podman machine ssh -- "sudo podman build \
  --platform linux/amd64 \
  -t ${IMAGE_NAME}:${IMAGE_TAG} \
  ${VM_BUILD_DIR}"

# ---- Step 2: Prepare output directory ----
mkdir -p "${OUTPUT_DIR}"
podman machine ssh -- "sudo mkdir -p /tmp/rhel-output"

# ---- Step 3: Generate the Anaconda ISO ----
echo ">>> Generating anaconda-iso via bootc-image-builder..."
podman machine ssh -- "sudo podman run \
  --rm \
  --privileged \
  --pull=newer \
  --security-opt label=type:unconfined_t \
  -v ${VM_BUILD_DIR}/config.toml:/config.toml:ro \
  -v /tmp/rhel-output:/output \
  -v /var/lib/containers/storage:/var/lib/containers/storage \
  registry.redhat.io/rhel10/bootc-image-builder:latest \
  --local \
  --type anaconda-iso \
  --config /config.toml \
  ${IMAGE_NAME}:${IMAGE_TAG}"

# ---- Step 4: Copy ISO back to macOS ----
echo ">>> Copying ISO to ${OUTPUT_DIR}..."
podman machine ssh -- "sudo find /tmp/rhel-output -name '*.iso'" | while read -r iso; do
  podman machine ssh -- "sudo cat $iso" > "${OUTPUT_DIR}/$(basename "$iso")"
done

echo ""
echo ">>> Build complete!"
echo ">>> ISO is in: ${OUTPUT_DIR}/"
ls -lh "${OUTPUT_DIR}"/*.iso 2>/dev/null || echo "(check ${OUTPUT_DIR} for the .iso file)"
