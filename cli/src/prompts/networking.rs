use anyhow::Result;
use dialoguer::{Confirm, Input};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    cfg.rke2.networking.cluster_cidr = Input::new()
        .with_prompt("Cluster CIDR (pod IPs)")
        .default("10.42.0.0/16".into())
        .interact_text()?;

    cfg.rke2.networking.service_cidr = Input::new()
        .with_prompt("Service CIDR")
        .default("10.43.0.0/16".into())
        .interact_text()?;

    cfg.rke2.networking.cluster_dns = Input::new()
        .with_prompt("Cluster DNS IP")
        .default("10.43.0.10".into())
        .interact_text()?;

    cfg.rke2.networking.cluster_domain = Input::new()
        .with_prompt("Cluster domain")
        .default("cluster.local".into())
        .interact_text()?;

    cfg.rke2.networking.service_node_port_range = Input::new()
        .with_prompt("NodePort range")
        .default("30000-32767".into())
        .interact_text()?;

    cfg.rke2.networking.disable_ipv6 = Confirm::new()
        .with_prompt("Disable IPv6?")
        .default(true)
        .interact()?;

    cfg.rke2.networking.dual_stack = Confirm::new()
        .with_prompt("Enable dual-stack (IPv4+IPv6)?")
        .default(false)
        .interact()?;

    if cfg.rke2.networking.dual_stack {
        cfg.rke2.networking.dual_stack_cluster_cidr = Input::new()
            .with_prompt("IPv6 cluster CIDR")
            .default("fd00:42::/56".into())
            .interact_text()?;
        cfg.rke2.networking.dual_stack_service_cidr = Input::new()
            .with_prompt("IPv6 service CIDR")
            .default("fd00:43::/112".into())
            .interact_text()?;
    }
    Ok(())
}
