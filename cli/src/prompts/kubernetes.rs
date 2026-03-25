use anyhow::Result;
use dialoguer::Select;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let versions = &["1.30", "1.31", "1.32", "1.33"];
    let idx = Select::new()
        .with_prompt("RKE2 Kubernetes version")
        .items(versions)
        .default(2) // 1.32
        .interact()?;
    cfg.rke2.kubernetes.rke2_version = versions[idx].into();

    let channels = &["stable", "latest", "testing"];
    let idx = Select::new()
        .with_prompt("Release channel")
        .items(channels)
        .default(0)
        .interact()?;
    cfg.rke2.kubernetes.channel = channels[idx].into();

    let cnis = &["canal", "calico", "cilium", "flannel", "multus", "none"];
    let idx = Select::new()
        .with_prompt("CNI plugin")
        .items(cnis)
        .default(0)
        .interact()?;
    cfg.rke2.cni = match idx {
        0 => crate::config::Cni::Canal,
        1 => crate::config::Cni::Calico,
        2 => crate::config::Cni::Cilium,
        3 => crate::config::Cni::Flannel,
        4 => crate::config::Cni::Multus,
        5 => crate::config::Cni::None,
        _ => crate::config::Cni::Canal,
    };
    Ok(())
}
