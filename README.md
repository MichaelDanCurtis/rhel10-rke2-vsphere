# RHEL 10 Image Mode — RKE2 Kubernetes for vSphere

Immutable RHEL 10 bootc image with RKE2 (Rancher Kubernetes) baked in,
packaged as a bootable Anaconda ISO (or VMDK/QCOW2/AMI) for deployment.

## Quick Start

```bash
# Build and run the configurator wizard
cd cli
cargo run

# The wizard walks you through all RKE2 + RHEL 10 options, generates
# the build files, and optionally pushes to GitHub to trigger CI.
```

## How It Works

1. **Run the CLI wizard** — interactive prompts for all RKE2 and RHEL 10 options
2. **Config files are generated** — `Containerfile`, `config.toml`, `rke2-config.yaml`, etc.
3. **Push to GitHub** — triggers the CI workflow on an x86_64 runner
4. **Download the artifact** — ISO/VMDK/QCOW2 is uploaded to GitHub Releases

## Configurable Options

### RKE2 / Kubernetes
- **Node role**: server, agent, dedicated etcd, dedicated control-plane
- **K8s version**: 1.30-1.33, stable/latest/testing channels
- **CNI**: canal, calico, cilium, flannel, multus, none
- **Networking**: cluster/service CIDRs, DNS, IPv6, dual-stack, NodePort range
- **TLS**: Subject Alternative Names, kubeconfig mode
- **Cluster join**: server URL, token
- **Node**: name, IP, labels, taints
- **etcd**: snapshots, cron schedule, retention, S3 backup
- **Security**: CIS benchmark profile, SELinux
- **Components**: disable coredns/ingress/metrics, ingress controller, ServiceLB
- **Cloud provider**: none, aws, vsphere, custom

### RHEL 10 OS
- **Image**: base image, output type (ISO/VMDK/QCOW2/AMI/VHD/GCE), hostname
- **Disk**: root size, /var/lib/rancher size, partition mode
- **Users**: admin user, password, SSH keys
- **Packages**: extra RPMs, systemd services, embedded containers
- **Network**: DHCP or static IP configuration
- **Firewall**: firewalld ports, services, default zone
- **Timezone**: timezone, NTP servers, locale, keyboard
- **Kernel**: boot arguments, kernel package override (kernel-rt), FIPS mode
- **Hardening**: OpenSCAP profiles (CIS, STIG, PCI-DSS, HIPAA, E8), SELinux mode

## GitHub Actions CI

The workflow runs on pushes to `configs/` and:

1. Logs into `registry.redhat.io`
2. Registers with `subscription-manager` (activation key)
3. Builds the bootc container image
4. Runs `bootc-image-builder` to produce the selected output type
5. Uploads the artifact to GitHub Releases

### Required Secrets

Set these in your GitHub repo settings or via `gh secret set`:

- `REDHAT_USERNAME` — Red Hat registry username
- `REDHAT_PASSWORD` — Red Hat registry password
- `RHEL_ORG_ID` — subscription-manager org ID
- `RHEL_ACTIVATION_KEY` — subscription-manager activation key

## Deploy to vSphere

1. Download the ISO from GitHub Releases
2. Upload to a vSphere datastore
3. Create VM: RHEL 10 64-bit, 4+ vCPU, 8+ GB RAM, 100+ GB disk
4. Mount ISO, power on — Anaconda runs unattended
5. SSH in: `ssh admin@<vm-ip>`
6. Verify: `sudo systemctl status rke2-server`
