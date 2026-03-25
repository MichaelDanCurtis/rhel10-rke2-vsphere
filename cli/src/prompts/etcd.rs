use anyhow::Result;
use dialoguer::{Confirm, Input};
use crate::config::{ImageConfig, NodeRole};

pub fn prompt(cfg: &mut ImageConfig) -> Result<()> {
    if cfg.rke2.node_role == NodeRole::Agent { return Ok(()); }

    cfg.rke2.etcd.snapshots_enabled = Confirm::new()
        .with_prompt("Enable etcd snapshots?")
        .default(true)
        .interact()?;

    if cfg.rke2.etcd.snapshots_enabled {
        cfg.rke2.etcd.snapshot_schedule_cron = Input::new()
            .with_prompt("Snapshot schedule (cron)")
            .default("0 */12 * * *".into())
            .interact_text()?;
        cfg.rke2.etcd.snapshot_retention = Input::new()
            .with_prompt("Snapshot retention count")
            .default(5)
            .interact()?;
        cfg.rke2.etcd.s3_backup = Confirm::new()
            .with_prompt("Enable S3 snapshot backup?")
            .default(false)
            .interact()?;
        if cfg.rke2.etcd.s3_backup {
            cfg.rke2.etcd.s3_endpoint = Input::new().with_prompt("S3 endpoint").default("s3.amazonaws.com".into()).interact_text()?;
            cfg.rke2.etcd.s3_bucket = Input::new().with_prompt("S3 bucket").interact_text()?;
            cfg.rke2.etcd.s3_region = Input::new().with_prompt("S3 region").default("us-east-1".into()).interact_text()?;
            cfg.rke2.etcd.s3_access_key = Input::new().with_prompt("S3 access key").interact_text()?;
            cfg.rke2.etcd.s3_secret_key = Input::new().with_prompt("S3 secret key").interact_text()?;
        }
    }
    Ok(())
}
