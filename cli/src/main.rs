mod config;
mod git;
mod prompts;
mod templates;

use anyhow::Result;
use dialoguer::Confirm;

fn main() -> Result<()> {
    // Run the interactive wizard
    let cfg = prompts::run_wizard()?;

    // Determine output directory (repo root)
    let repo_dir = std::env::current_dir()?
        .ancestors()
        .find(|p| p.join(".git").exists())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("\n>>> Generating build files in: {}", repo_dir.display());
    templates::generate_all(&cfg, &repo_dir)?;

    println!("✓ Generated:");
    println!("  • Containerfile");
    println!("  • config.toml");
    println!("  • files/rke2-config.yaml");
    println!("  • files/rancher-rke2.repo");
    println!("  • configs/build.json");
    println!("  • configs/image-config.json");

    // Ask to push
    let push = Confirm::new()
        .with_prompt("Commit and push to GitHub to trigger CI build?")
        .default(true)
        .interact()?;

    if push {
        git::commit_and_push(&repo_dir)?;
    } else {
        println!("\nFiles generated locally. Run `git add -A && git commit && git push` when ready.");
    }

    Ok(())
}
