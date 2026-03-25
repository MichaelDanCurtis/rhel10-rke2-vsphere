use anyhow::Result;
use dialoguer::{Confirm, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let profiles = &["none", "cis", "cis-1.23"];
    let idx = Select::new()
        .with_prompt("CIS benchmark profile")
        .items(profiles)
        .default(0)
        .interact()?;
    cfg.rke2.security.cis_profile = profiles[idx].into();

    cfg.rke2.security.selinux = Confirm::new()
        .with_prompt("Enable SELinux for RKE2?")
        .default(false)
        .interact()?;
    Ok(())
}
