use anyhow::Result;
use dialoguer::Select;
use crate::config::{ImageConfig, NodeRole};

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let roles = &[
        "Server (control plane + etcd + worker)",
        "Agent (worker only)",
        "Dedicated etcd node",
        "Dedicated control-plane node",
    ];
    let idx = Select::new()
        .with_prompt("Node role")
        .items(roles)
        .default(0)
        .interact()?;

    cfg.rke2.node_role = match idx {
        0 => NodeRole::Server,
        1 => NodeRole::Agent,
        2 => NodeRole::DedicatedEtcd,
        3 => NodeRole::DedicatedControlPlane,
        _ => NodeRole::Server,
    };
    Ok(())
}
