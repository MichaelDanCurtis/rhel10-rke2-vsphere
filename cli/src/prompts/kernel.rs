use anyhow::Result;
use dialoguer::{Confirm, Input};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let kargs: String = Input::new()
        .with_prompt("Extra kernel boot arguments (space-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.kernel.extra_kargs = kargs.split_whitespace().map(|s| s.to_string()).collect();

    cfg.os.kernel.kernel_package = Input::new()
        .with_prompt("Kernel package override (e.g. kernel-rt, or empty for default)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;

    cfg.os.kernel.fips = Confirm::new()
        .with_prompt("Enable FIPS mode?")
        .default(false)
        .interact()?;
    Ok(())
}
