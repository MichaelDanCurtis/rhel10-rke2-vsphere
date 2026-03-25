use anyhow::Result;
use dialoguer::Input;
use crate::config::{ImageConfig, NodeRole};

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    if cfg.rke2.node_role == NodeRole::Agent || cfg.rke2.node_role == NodeRole::DedicatedControlPlane {
        cfg.rke2.cluster_join.server_url = Input::new()
            .with_prompt("Server URL to join (e.g. https://10.0.0.1:9345)")
            .interact_text()?;
        cfg.rke2.cluster_join.token = Input::new()
            .with_prompt("Cluster token")
            .interact_text()?;
    } else {
        let token: String = Input::new()
            .with_prompt("Cluster token (leave empty to auto-generate)")
            .default(String::new())
            .allow_empty(true)
            .interact_text()?;
        cfg.rke2.cluster_join.token = token;
    }
    Ok(())
}
