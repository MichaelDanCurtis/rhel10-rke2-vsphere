use anyhow::Result;
use dialoguer::{Confirm, MultiSelect, Select};
use crate::config::ImageConfig;

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    let components = &["rke2-coredns", "rke2-ingress-nginx", "rke2-metrics-server"];
    let disabled = MultiSelect::new()
        .with_prompt("Disable bundled components (space to toggle, enter to confirm)")
        .items(components)
        .interact()?;
    cfg.rke2.components.disable = disabled.iter().map(|&i| components[i].to_string()).collect();

    let ingress = &["ingress-nginx", "traefik", "none"];
    let idx = Select::new()
        .with_prompt("Ingress controller")
        .items(ingress)
        .default(0)
        .interact()?;
    cfg.rke2.components.ingress_controller = ingress[idx].into();

    cfg.rke2.components.enable_servicelb = Confirm::new()
        .with_prompt("Enable ServiceLB (built-in load balancer)?")
        .default(false)
        .interact()?;
    Ok(())
}
