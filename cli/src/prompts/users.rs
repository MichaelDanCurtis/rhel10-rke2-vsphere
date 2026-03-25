use anyhow::Result;
use dialoguer::Input;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.os.users.admin_username = Input::new()
        .with_prompt("Admin username")
        .default("admin".into())
        .interact_text()?;

    cfg.os.users.admin_password = Input::new()
        .with_prompt("Admin password")
        .default("changeme".into())
        .interact_text()?;

    let keys: String = Input::new()
        .with_prompt("SSH public key(s) (comma-separated, or empty)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.users.ssh_keys = keys.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    Ok(())
}
