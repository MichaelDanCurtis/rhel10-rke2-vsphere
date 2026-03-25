use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result};

/// Stage, commit, and push all generated files to trigger the CI build.
pub fn commit_and_push(repo_dir: &Path) -> Result<()> {
    let run = |args: &[&str]| -> Result<()> {
        let status = Command::new("git")
            .args(args)
            .current_dir(repo_dir)
            .status()
            .with_context(|| format!("failed to run: git {}", args.join(" ")))?;
        if !status.success() {
            anyhow::bail!("git {} failed with {}", args.join(" "), status);
        }
        Ok(())
    };

    run(&["add", "-A"])?;
    run(&["commit", "-m", "chore: update image configuration\n\nCo-Authored-By: Oz <oz-agent@warp.dev>"])?;
    run(&["push", "origin", "main"])?;

    println!("\n✅ Config pushed to GitHub — CI build will start automatically.");
    println!("   Watch progress at: https://github.com/$(git remote get-url origin | sed 's|.*github.com[:/]||;s|\\.git$||')/actions");
    Ok(())
}
