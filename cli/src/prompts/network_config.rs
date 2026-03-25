use anyhow::Result;
use dialoguer::{Input, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let methods = &["dhcp", "static"];
    let idx = Select::new()
        .with_prompt("Network method")
        .items(methods)
        .default(0)
        .interact()?;
    cfg.os.network.method = methods[idx].into();

    if cfg.os.network.method == "static" {
        cfg.os.network.static_ip = Input::new().with_prompt("Static IP (e.g. 192.168.1.100)").interact_text()?;
        cfg.os.network.netmask = Input::new().with_prompt("Netmask").default("255.255.255.0".into()).interact_text()?;
        cfg.os.network.gateway = Input::new().with_prompt("Gateway").interact_text()?;
        let dns: String = Input::new().with_prompt("DNS servers (comma-separated)").default("8.8.8.8".into()).interact_text()?;
        cfg.os.network.dns = dns.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        cfg.os.network.interface = Input::new().with_prompt("Interface").default("link".into()).interact_text()?;
    }
    Ok(())
}
