use anyhow::Result;
use dialoguer::Input;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.rke2.node.node_name = Input::new()
        .with_prompt("Node name (empty = use hostname)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;

    cfg.rke2.node.node_ip = Input::new()
        .with_prompt("Node IP (empty = auto-detect)")
        .default("0.0.0.0".into())
        .interact_text()?;

    let labels: String = Input::new()
        .with_prompt("Node labels (comma-separated key=value, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.rke2.node.node_labels = labels.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    let taints: String = Input::new()
        .with_prompt("Node taints (comma-separated key=val:effect, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.rke2.node.node_taints = taints.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    Ok(())
}
