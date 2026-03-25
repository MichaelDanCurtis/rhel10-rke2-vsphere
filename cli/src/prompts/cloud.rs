use anyhow::Result;
use dialoguer::{Input, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let providers = &["none", "aws", "vsphere", "custom"];
    let idx = Select::new()
        .with_prompt("Cloud provider")
        .items(providers)
        .default(0)
        .interact()?;
    cfg.rke2.cloud_provider.name = providers[idx].into();

    if cfg.rke2.cloud_provider.name != "none" && cfg.rke2.cloud_provider.name != "" {
        cfg.rke2.cloud_provider.config_path = Input::new()
            .with_prompt("Cloud provider config path (empty if N/A)")
            .default(String::new())
            .allow_empty(true)
            .interact_text()?;
    }
    Ok(())
}
