use anyhow::Result;
use dialoguer::Input;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let san: String = Input::new()
        .with_prompt("TLS SANs (comma-separated hostnames/IPs, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.rke2.tls.tls_san = san.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    cfg.rke2.tls.write_kubeconfig_mode = Input::new()
        .with_prompt("kubeconfig file mode")
        .default("0644".into())
        .interact_text()?;
    Ok(())
}
