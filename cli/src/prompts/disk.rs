use anyhow::Result;
use dialoguer::{Input, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.os.disk.root_size = Input::new()
        .with_prompt("Root (/) filesystem size")
        .default("20 GiB".into())
        .interact_text()?;

    cfg.os.disk.rancher_size = Input::new()
        .with_prompt("/var/lib/rancher size")
        .default("60 GiB".into())
        .interact_text()?;

    let modes = &["auto-lvm", "plain", "lvm", "btrfs"];
    let idx = Select::new()
        .with_prompt("Partition mode")
        .items(modes)
        .default(0)
        .interact()?;
    cfg.os.disk.partition_mode = modes[idx].into();
    Ok(())
}
