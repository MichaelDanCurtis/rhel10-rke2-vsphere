use anyhow::Result;
use dialoguer::Input;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let pkgs: String = Input::new()
        .with_prompt("Extra RPM packages (comma-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.packages.extra_packages = pkgs.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    let enable: String = Input::new()
        .with_prompt("Systemd services to enable (comma-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.packages.enable_services = enable.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    let disable: String = Input::new()
        .with_prompt("Systemd services to disable (comma-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.packages.disable_services = disable.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    let containers: String = Input::new()
        .with_prompt("Embed containers (image refs, comma-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.packages.embedded_containers = containers.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    Ok(())
}
