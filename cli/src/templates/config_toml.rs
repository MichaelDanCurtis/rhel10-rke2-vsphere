use crate::config::ImageConfig;

pub fn render(cfg: &ImageConfig) -> String {
    let mut lines = Vec::new();
    lines.push("# bootc-image-builder configuration (auto-generated)".into());
    lines.push(String::new());

    // Filesystem
    lines.push("[[customizations.filesystem]]".into());
    lines.push("mountpoint = \"/\"".into());
    lines.push(format!("minsize = \"{}\"", cfg.os.disk.root_size));
    lines.push(String::new());

    lines.push("[[customizations.filesystem]]".into());
    lines.push("mountpoint = \"/var/lib/rancher\"".into());
    lines.push(format!("minsize = \"{}\"", cfg.os.disk.rancher_size));
    lines.push(String::new());

    for mp in &cfg.os.disk.extra_mounts {
        lines.push("[[customizations.filesystem]]".into());
        lines.push(format!("mountpoint = \"{}\"", mp.path));
        lines.push(format!("minsize = \"{}\"", mp.size));
        lines.push(String::new());
    }

    // Kickstart
    let mut ks = Vec::new();
    ks.push("text --non-interactive".into());
    ks.push("zerombr".into());
    ks.push("clearpart --all --initlabel --disklabel=gpt".into());

    let autopart = match cfg.os.disk.partition_mode.as_str() {
        "lvm" | "auto-lvm" => "autopart --noswap --type=lvm",
        "btrfs" => "autopart --noswap --type=btrfs",
        _ => "autopart --noswap",
    };
    ks.push(autopart.into());

    // Network
    if cfg.os.network.method == "static" {
        let dns_str = if !cfg.os.network.dns.is_empty() {
            format!(" --nameserver={}", cfg.os.network.dns.join(","))
        } else {
            String::new()
        };
        ks.push(format!(
            "network --bootproto=static --ip={} --netmask={} --gateway={}{} --device={} --activate --onboot=on",
            cfg.os.network.static_ip, cfg.os.network.netmask, cfg.os.network.gateway, dns_str, cfg.os.network.interface
        ));
    } else {
        ks.push("network --bootproto=dhcp --device=link --activate --onboot=on".into());
    }

    // User
    ks.push(format!(
        "user --name={} --groups=wheel --password={} --plaintext",
        cfg.os.users.admin_username, cfg.os.users.admin_password
    ));

    // SSH keys
    for key in &cfg.os.users.ssh_keys {
        ks.push(format!("sshkey --username={} \"{}\"", cfg.os.users.admin_username, key));
    }

    // Timezone
    if cfg.os.timezone.timezone != "UTC" {
        ks.push(format!("timezone {} --utc", cfg.os.timezone.timezone));
    }

    // Keyboard
    if cfg.os.timezone.keyboard != "us" {
        ks.push(format!("keyboard --xlayouts='{}'", cfg.os.timezone.keyboard));
    }

    // Locale
    if cfg.os.timezone.locale != "en_US.UTF-8" {
        ks.push(format!("lang {}", cfg.os.timezone.locale));
    }

    lines.push("[customizations.installer.kickstart]".into());
    lines.push(format!("contents = \"\"\"\n{}\n\"\"\"", ks.join("\n")));
    lines.push(String::new());

    lines.join("\n")
}
