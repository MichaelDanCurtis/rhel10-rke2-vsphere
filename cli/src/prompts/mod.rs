pub mod node_role;
pub mod kubernetes;
pub mod networking;
pub mod tls;
pub mod cluster;
pub mod node;
pub mod etcd;
pub mod rke2_security;
pub mod components;
pub mod cloud;
pub mod os_image;
pub mod disk;
pub mod users;
pub mod packages;
pub mod network_config;
pub mod firewall;
pub mod timezone;
pub mod kernel;
pub mod hardening;

use anyhow::Result;
use crate::config::ImageConfig;

/// Run the full interactive wizard and return a populated config.
pub fn run_wizard() -> Result<ImageConfig> {
    let mut cfg = ImageConfig::default();

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║   RHEL 10 RKE2 Image Configurator            ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // ── RKE2 / Kubernetes ──
    println!("─── RKE2 / Kubernetes Configuration ───\n");
    node_role::prompt(&mut cfg)?;
    kubernetes::prompt(&mut cfg)?;
    networking::prompt(&mut cfg)?;
    tls::prompt(&mut cfg)?;
    cluster::prompt(&mut cfg)?;
    node::prompt(&mut cfg)?;
    etcd::prompt(&mut cfg)?;
    rke2_security::prompt(&mut cfg)?;
    components::prompt(&mut cfg)?;
    cloud::prompt(&mut cfg)?;

    // ── RHEL 10 OS ──
    println!("\n─── RHEL 10 OS Configuration ───\n");
    os_image::prompt(&mut cfg)?;
    disk::prompt(&mut cfg)?;
    users::prompt(&mut cfg)?;
    packages::prompt(&mut cfg)?;
    network_config::prompt(&mut cfg)?;
    firewall::prompt(&mut cfg)?;
    timezone::prompt(&mut cfg)?;
    kernel::prompt(&mut cfg)?;
    hardening::prompt(&mut cfg)?;

    Ok(cfg)
}
