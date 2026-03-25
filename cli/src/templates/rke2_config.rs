use crate::config::{ImageConfig, NodeRole, Cni};

pub fn render(cfg: &ImageConfig) -> String {
    let mut lines = Vec::new();
    lines.push("# /etc/rancher/rke2/config.yaml (auto-generated)".into());
    lines.push(String::new());

    // Cluster join
    if !cfg.rke2.cluster_join.server_url.is_empty() {
        lines.push(format!("server: \"{}\"", cfg.rke2.cluster_join.server_url));
    }
    if !cfg.rke2.cluster_join.token.is_empty() {
        lines.push(format!("token: \"{}\"", cfg.rke2.cluster_join.token));
    }

    // Networking
    lines.push(format!("node-ip: \"{}\"", cfg.rke2.node.node_ip));
    if cfg.rke2.networking.dual_stack {
        lines.push(format!("cluster-cidr: \"{},{}\"", cfg.rke2.networking.cluster_cidr, cfg.rke2.networking.dual_stack_cluster_cidr));
        lines.push(format!("service-cidr: \"{},{}\"", cfg.rke2.networking.service_cidr, cfg.rke2.networking.dual_stack_service_cidr));
    } else {
        lines.push(format!("cluster-cidr: \"{}\"", cfg.rke2.networking.cluster_cidr));
        lines.push(format!("service-cidr: \"{}\"", cfg.rke2.networking.service_cidr));
    }
    lines.push(format!("cluster-dns: \"{}\"", cfg.rke2.networking.cluster_dns));
    lines.push(format!("cluster-domain: \"{}\"", cfg.rke2.networking.cluster_domain));
    lines.push(format!("service-node-port-range: \"{}\"", cfg.rke2.networking.service_node_port_range));
    lines.push(String::new());

    // kubeconfig
    lines.push(format!("write-kubeconfig-mode: \"{}\"", cfg.rke2.tls.write_kubeconfig_mode));
    lines.push(String::new());

    // CNI
    let cni_str = match &cfg.rke2.cni {
        Cni::Canal => "canal",
        Cni::Calico => "calico",
        Cni::Cilium => "cilium",
        Cni::Flannel => "flannel",
        Cni::Multus => "multus",
        Cni::None => "none",
    };
    lines.push("cni:".into());
    lines.push(format!("  - {}", cni_str));
    lines.push(String::new());

    // TLS SANs
    if !cfg.rke2.tls.tls_san.is_empty() {
        lines.push("tls-san:".into());
        for san in &cfg.rke2.tls.tls_san {
            lines.push(format!("  - \"{}\"", san));
        }
        lines.push(String::new());
    }

    // Node name
    if !cfg.rke2.node.node_name.is_empty() {
        lines.push(format!("node-name: \"{}\"", cfg.rke2.node.node_name));
    }

    // Node labels
    if !cfg.rke2.node.node_labels.is_empty() {
        lines.push("node-label:".into());
        for l in &cfg.rke2.node.node_labels {
            lines.push(format!("  - \"{}\"", l));
        }
    }

    // Node taints
    if !cfg.rke2.node.node_taints.is_empty() {
        lines.push("node-taint:".into());
        for t in &cfg.rke2.node.node_taints {
            lines.push(format!("  - \"{}\"", t));
        }
    }

    // Dedicated roles
    match cfg.rke2.node_role {
        NodeRole::DedicatedEtcd => {
            lines.push("disable-apiserver: true".into());
            lines.push("disable-controller-manager: true".into());
            lines.push("disable-scheduler: true".into());
        }
        NodeRole::DedicatedControlPlane => {
            lines.push("disable-etcd: true".into());
        }
        _ => {}
    }

    // etcd snapshots
    if cfg.rke2.node_role != NodeRole::Agent {
        if !cfg.rke2.etcd.snapshots_enabled {
            lines.push("etcd-disable-snapshots: true".into());
        } else {
            lines.push(format!("etcd-snapshot-schedule-cron: \"{}\"", cfg.rke2.etcd.snapshot_schedule_cron));
            lines.push(format!("etcd-snapshot-retention: {}", cfg.rke2.etcd.snapshot_retention));
            if cfg.rke2.etcd.s3_backup {
                lines.push("etcd-s3: true".into());
                lines.push(format!("etcd-s3-endpoint: \"{}\"", cfg.rke2.etcd.s3_endpoint));
                lines.push(format!("etcd-s3-bucket: \"{}\"", cfg.rke2.etcd.s3_bucket));
                lines.push(format!("etcd-s3-region: \"{}\"", cfg.rke2.etcd.s3_region));
                lines.push(format!("etcd-s3-access-key: \"{}\"", cfg.rke2.etcd.s3_access_key));
                lines.push(format!("etcd-s3-secret-key: \"{}\"", cfg.rke2.etcd.s3_secret_key));
            }
        }
        lines.push(String::new());
    }

    // Security
    if cfg.rke2.security.cis_profile != "none" {
        lines.push(format!("profile: \"{}\"", cfg.rke2.security.cis_profile));
    }
    if cfg.rke2.security.selinux {
        lines.push("selinux: true".into());
    }

    // Disabled components
    if !cfg.rke2.components.disable.is_empty() {
        lines.push("disable:".into());
        for c in &cfg.rke2.components.disable {
            lines.push(format!("  - \"{}\"", c));
        }
    }

    // ServiceLB
    if cfg.rke2.components.enable_servicelb {
        lines.push("enable-servicelb: true".into());
    }

    // Cloud provider
    if cfg.rke2.cloud_provider.name != "none" && !cfg.rke2.cloud_provider.name.is_empty() {
        lines.push(format!("cloud-provider-name: \"{}\"", cfg.rke2.cloud_provider.name));
        if !cfg.rke2.cloud_provider.config_path.is_empty() {
            lines.push(format!("cloud-provider-config: \"{}\"", cfg.rke2.cloud_provider.config_path));
        }
    }

    lines.push(String::new());
    lines.join("\n")
}
