use anyhow::Result;
use dialoguer::{Input, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.os.image.base_image = Input::new()
        .with_prompt("Base image")
        .default("registry.redhat.io/rhel10/rhel-bootc:latest".into())
        .interact_text()?;

    let types = &["anaconda-iso", "vmdk", "qcow2", "raw", "ami", "vhd", "gce"];
    let idx = Select::new()
        .with_prompt("Output image type")
        .items(types)
        .default(0)
        .interact()?;
    cfg.os.image.output_type = types[idx].into();

    cfg.os.image.hostname = Input::new()
        .with_prompt("Hostname (empty = default)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;

    cfg.os.image.image_version = Input::new()
        .with_prompt("Image version label (empty = default)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    Ok(())
}
