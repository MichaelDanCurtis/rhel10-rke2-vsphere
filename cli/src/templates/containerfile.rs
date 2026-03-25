use crate::config::{ImageConfig, NodeRole};

pub fn render(cfg: &ImageConfig) -> String {
    let mut lines = Vec::new();
    let base = &cfg.os.image.base_image;

    lines.push(format!("FROM {base}"));
    lines.push(String::new());

    // ── Image version label ──
    if !cfg.os.image.image_version.is_empty() {
        lines.push(format!("LABEL org.opencontainers.image.version=\"{}\"", cfg.os.image.image_version));
        lines.push(String::new());
    }

    // ── Hostname ──
    if !cfg.os.image.hostname.is_empty() {
        lines.push(format!("RUN echo \"{}\" > /etc/hostname", cfg.os.image.hostname));
        lines.push(String::new());
    }

    // ── Kernel override ──
    if !cfg.os.kernel.kernel_package.is_empty() {
        lines.push(format!("RUN dnf -y install {} && dnf clean all", cfg.os.kernel.kernel_package));
        lines.push(String::new());
    }

    // ── VMware vSphere guest integration ──
    lines.push("# ----- VMware vSphere guest integration -----".into());
    lines.push("RUN dnf -y install open-vm-tools && \\".into());
    lines.push("    systemctl enable vmtoolsd.service".into());
    lines.push(String::new());

    // ── Kernel modules for RKE2 ──
    lines.push("RUN dnf -y install kernel-modules-extra".into());
    lines.push(String::new());

    // ── Networking utilities ──
    lines.push("RUN dnf -y install NetworkManager iptables-nft container-selinux".into());
    lines.push(String::new());

    // ── Firewall ──
    if cfg.os.firewall.enabled {
        let mut fw_cmds = vec!["RUN dnf -y install firewalld && \\".to_string()];
        fw_cmds.push("    systemctl enable firewalld && \\".into());
        for port in &cfg.os.firewall.open_ports {
            fw_cmds.push(format!("    firewall-offline-cmd --add-port={} && \\", port));
        }
        for svc in &cfg.os.firewall.open_services {
            fw_cmds.push(format!("    firewall-offline-cmd --add-service={} && \\", svc));
        }
        // Remove trailing " && \"
        if let Some(last) = fw_cmds.last_mut() {
            *last = last.trim_end_matches(" && \\").to_string();
        }
        lines.extend(fw_cmds);
        lines.push(String::new());
    }

    // ── Extra packages ──
    if !cfg.os.packages.extra_packages.is_empty() {
        lines.push(format!("RUN dnf -y install {} && dnf clean all", cfg.os.packages.extra_packages.join(" ")));
        lines.push(String::new());
    }

    // ── OpenSCAP hardening ──
    if cfg.os.hardening.oscap_profile != "none" {
        lines.push("RUN dnf -y install openscap-utils scap-security-guide && \\".into());
        let profile_id = match cfg.os.hardening.oscap_profile.as_str() {
            "cis" => "xccdf_org.ssgproject.content_profile_cis",
            "stig" => "xccdf_org.ssgproject.content_profile_stig",
            "pci-dss" => "xccdf_org.ssgproject.content_profile_pci-dss",
            "hipaa" => "xccdf_org.ssgproject.content_profile_hipaa",
            "e8" => "xccdf_org.ssgproject.content_profile_e8",
            other => other,
        };
        lines.push(format!("    oscap-im --profile {} /usr/share/xml/scap/ssg/content/ssg-rhel10-ds.xml", profile_id));
        lines.push(String::new());
    }

    // ── FIPS mode ──
    if cfg.os.kernel.fips {
        lines.push("RUN mkdir -p /usr/lib/bootc/kargs.d && \\".into());
        lines.push("    echo 'kargs = [\"fips=1\"]' > /usr/lib/bootc/kargs.d/01-fips.toml && \\".into());
        lines.push("    update-crypto-policies --no-reload --set FIPS".into());
        lines.push(String::new());
    }

    // ── Enable/disable services ──
    for svc in &cfg.os.packages.enable_services {
        lines.push(format!("RUN systemctl enable {}.service", svc));
    }
    for svc in &cfg.os.packages.disable_services {
        lines.push(format!("RUN systemctl disable {}.service", svc));
    }
    if !cfg.os.packages.enable_services.is_empty() || !cfg.os.packages.disable_services.is_empty() {
        lines.push(String::new());
    }

    // ── Embedded containers ──
    for img in &cfg.os.packages.embedded_containers {
        lines.push(format!("RUN podman pull {}", img));
    }
    if !cfg.os.packages.embedded_containers.is_empty() {
        lines.push(String::new());
    }

    // ── Timezone & locale ──
    if cfg.os.timezone.timezone != "UTC" {
        lines.push(format!("RUN ln -sf /usr/share/zoneinfo/{} /etc/localtime", cfg.os.timezone.timezone));
    }
    if cfg.os.timezone.locale != "en_US.UTF-8" {
        lines.push(format!("RUN echo 'LANG=\"{}\"' > /etc/locale.conf", cfg.os.timezone.locale));
    }
    if cfg.os.timezone.keyboard != "us" {
        lines.push(format!("RUN echo 'KEYMAP={}' > /etc/vconsole.conf", cfg.os.timezone.keyboard));
    }

    // ── RKE2 ──
    lines.push(String::new());
    lines.push("# ----- RKE2 -----".into());
    lines.push("COPY files/rancher-rke2.repo /etc/yum.repos.d/rancher-rke2.repo".into());

    let rke2_pkg = match cfg.rke2.node_role {
        NodeRole::Agent => "rke2-agent",
        _ => "rke2-server",
    };
    let rke2_svc = match cfg.rke2.node_role {
        NodeRole::Agent => "rke2-agent",
        _ => "rke2-server",
    };
    lines.push(format!("RUN dnf -y install {} && \\", rke2_pkg));
    lines.push(format!("    systemctl enable {}.service", rke2_svc));
    lines.push(String::new());

    lines.push("RUN mkdir -p /etc/rancher/rke2".into());
    lines.push("COPY files/rke2-config.yaml /etc/rancher/rke2/config.yaml".into());
    lines.push(String::new());

    // ── IPv6 disable ──
    if cfg.rke2.networking.disable_ipv6 {
        lines.push("RUN mkdir -p /usr/lib/bootc/kargs.d".into());
        lines.push("COPY files/01-disable-ipv6.toml /usr/lib/bootc/kargs.d/01-disable-ipv6.toml".into());
        lines.push("COPY files/99-disable-ipv6.conf /etc/sysctl.d/99-disable-ipv6.conf".into());
        lines.push(String::new());
    }

    // ── Extra kernel args ──
    if !cfg.os.kernel.extra_kargs.is_empty() {
        let args_json: Vec<String> = cfg.os.kernel.extra_kargs.iter().map(|a| format!("\"{}\"", a)).collect();
        lines.push("RUN mkdir -p /usr/lib/bootc/kargs.d".into());
        lines.push(format!("RUN echo 'kargs = [{}]' > /usr/lib/bootc/kargs.d/99-custom.toml", args_json.join(", ")));
        lines.push(String::new());
    }

    // ── SELinux mode ──
    if cfg.os.hardening.selinux_mode == "permissive" {
        lines.push("RUN sed -i 's/^SELINUX=.*/SELINUX=permissive/' /etc/selinux/config".into());
    } else if cfg.os.hardening.selinux_mode == "disabled" {
        lines.push("RUN sed -i 's/^SELINUX=.*/SELINUX=disabled/' /etc/selinux/config".into());
    }

    // ── Greenboot (edge resilience) ──
    if cfg.os.edge.greenboot {
        lines.push(String::new());
        lines.push("RUN dnf -y install greenboot greenboot-default-health-checks && \\".into());
        lines.push("    systemctl enable greenboot-healthcheck.service greenboot-task-runner.service".into());
    }

    // ── Cleanup ──
    lines.push(String::new());
    lines.push("# ----- Cleanup -----".into());
    lines.push("RUN dnf clean all && \\".into());
    lines.push("    rm -rf /var/cache/dnf /var/lib/dnf".into());
    lines.push(String::new());
    lines.push("CMD [\"/sbin/init\"]".into());
    lines.push(String::new());

    lines.join("\n")
}
