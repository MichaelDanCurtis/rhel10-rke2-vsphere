pub mod containerfile;
pub mod config_toml;
pub mod rke2_config;
pub mod rke2_repo;

use std::path::Path;
use std::fs;
use anyhow::Result;
use crate::config::ImageConfig;

/// Generate all build files from the config into the given output directory.
pub fn generate_all(cfg: &ImageConfig, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir.join("files"))?;
    fs::create_dir_all(out_dir.join("configs"))?;

    // Containerfile
    fs::write(out_dir.join("Containerfile"), containerfile::render(cfg))?;

    // config.toml (bootc-image-builder)
    fs::write(out_dir.join("config.toml"), config_toml::render(cfg))?;

    // RKE2 config
    fs::write(out_dir.join("files/rke2-config.yaml"), rke2_config::render(cfg))?;

    // RKE2 repo
    fs::write(out_dir.join("files/rancher-rke2.repo"), rke2_repo::render(cfg))?;

    // IPv6 disable files (if applicable)
    if cfg.rke2.networking.disable_ipv6 {
        fs::write(out_dir.join("files/01-disable-ipv6.toml"), "kargs = [\"ipv6.disable=1\"]\n")?;
        fs::write(out_dir.join("files/99-disable-ipv6.conf"),
            "net.ipv6.conf.all.disable_ipv6 = 1\nnet.ipv6.conf.default.disable_ipv6 = 1\nnet.ipv6.conf.lo.disable_ipv6 = 1\n")?;
    }

    // Build metadata (for CI)
    let build_json = serde_json::json!({
        "output_type": cfg.os.image.output_type,
        "image_name": "rhel10-rke2-vsphere",
    });
    fs::write(out_dir.join("configs/build.json"), serde_json::to_string_pretty(&build_json)?)?;

    // Full config JSON (for reproducibility)
    fs::write(out_dir.join("configs/image-config.json"), serde_json::to_string_pretty(cfg)?)?;

    Ok(())
}
