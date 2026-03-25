use anyhow::Result;
use dialoguer::Input;
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.os.timezone.timezone = Input::new()
        .with_prompt("Timezone")
        .default("UTC".into())
        .interact_text()?;

    let ntp: String = Input::new()
        .with_prompt("NTP servers (comma-separated, or empty for default)")
        .default(String::new())
        .allow_empty(true)
        .interact_text()?;
    cfg.os.timezone.ntp_servers = ntp.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    cfg.os.timezone.locale = Input::new()
        .with_prompt("Locale")
        .default("en_US.UTF-8".into())
        .interact_text()?;

    cfg.os.timezone.keyboard = Input::new()
        .with_prompt("Keyboard layout")
        .default("us".into())
        .interact_text()?;
    Ok(())
}
