use anyhow::Result;
use dialoguer::{Input, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let profiles = &["none", "cis", "stig", "pci-dss", "hipaa", "e8"];
    let idx = Select::new()
        .with_prompt("OpenSCAP hardening profile")
        .items(profiles)
        .default(0)
        .interact()?;
    cfg.os.hardening.oscap_profile = profiles[idx].into();

    if cfg.os.hardening.oscap_profile != "none" {
        cfg.os.hardening.tailoring_file = Input::new()
            .with_prompt("Custom tailoring file path (empty for default)")
            .default(String::new())
            .allow_empty(true)
            .interact_text()?;
    }

    let modes = &["enforcing", "permissive", "disabled"];
    let idx = Select::new()
        .with_prompt("SELinux mode")
        .items(modes)
        .default(0)
        .interact()?;
    cfg.os.hardening.selinux_mode = modes[idx].into();
    Ok(())
}
