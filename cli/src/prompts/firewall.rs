use anyhow::Result;
use dialoguer::{Confirm, Input};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.os.firewall.enabled = Confirm::new()
        .with_prompt("Enable firewalld?")
        .default(true)
        .interact()?;

    if cfg.os.firewall.enabled {
        let ports: String = Input::new()
            .with_prompt("Open ports (comma-separated, e.g. 22/tcp,6443/tcp)")
            .default("22/tcp,6443/tcp,9345/tcp,10250/tcp,2379-2380/tcp,30000-32767/tcp,30000-32767/udp".into())
            .interact_text()?;
        cfg.os.firewall.open_ports = ports.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

        let services: String = Input::new()
            .with_prompt("Open services (comma-separated, e.g. ssh,http)")
            .default("ssh".into())
            .interact_text()?;
        cfg.os.firewall.open_services = services.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

        cfg.os.firewall.default_zone = Input::new()
            .with_prompt("Default firewall zone")
            .default("public".into())
            .interact_text()?;
    }
    Ok(())
}
