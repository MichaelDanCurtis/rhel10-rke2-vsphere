# ============================================================
# RHEL 10 Image Mode — RKE2 Kubernetes Node for VMware vSphere
# Immutable bootc-based OS image
# ============================================================
FROM registry.redhat.io/rhel10/rhel-bootc:latest

# ----- RHEL 10 repo + entitlement certs (for building on non-RHEL hosts) -----
COPY files/entitlement/entitlement.pem /etc/pki/entitlement/entitlement.pem
COPY files/entitlement/entitlement-key.pem /etc/pki/entitlement/entitlement-key.pem
COPY files/entitlement/redhat-uep.pem /etc/rhsm/ca/redhat-uep.pem
COPY files/rhel10.repo /etc/yum.repos.d/rhel10.repo

# ----- VMware vSphere guest integration -----
RUN dnf -y install open-vm-tools && \
    systemctl enable vmtoolsd.service

# ----- RHEL 10 kernel modules required by RKE2 (nf_conntrack) -----
RUN dnf -y install kernel-modules-extra

# ----- Networking utilities (useful for debugging k8s networking) -----
RUN dnf -y install NetworkManager iptables-nft container-selinux

# ----- RKE2 repository -----
COPY files/rancher-rke2.repo /etc/yum.repos.d/rancher-rke2.repo

# ----- Install RKE2 server -----
# Switch to rke2-agent if this node will only be a worker
RUN dnf -y install rke2-server && \
    systemctl enable rke2-server.service

# ----- RKE2 configuration -----
RUN mkdir -p /etc/rancher/rke2
COPY files/rke2-config.yaml /etc/rancher/rke2/config.yaml

# ----- Disable IPv6 — kernel argument (primary method) -----
RUN mkdir -p /usr/lib/bootc/kargs.d
COPY files/01-disable-ipv6.toml /usr/lib/bootc/kargs.d/01-disable-ipv6.toml

# ----- Disable IPv6 — sysctl (defense in depth) -----
COPY files/99-disable-ipv6.conf /etc/sysctl.d/99-disable-ipv6.conf

# ----- Cleanup (remove build-time entitlement certs + dnf cache) -----
RUN dnf clean all && \
    rm -rf /var/cache/dnf /var/lib/dnf && \
    rm -f /etc/pki/entitlement/entitlement.pem /etc/pki/entitlement/entitlement-key.pem && \
    rm -f /etc/yum.repos.d/rhel10.repo

# ----- Container entrypoint (ignored when deployed to disk) -----
CMD ["/sbin/init"]
