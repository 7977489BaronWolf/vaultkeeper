use crate::config::Config;
use crate::store;
use anyhow::{bail, Context, Result};
use std::path::PathBuf;

/// Clone (duplicate) an existing vault environment into a new one.
pub fn run(source: &str, destination: &str, config: &Config) -> Result<()> {
    let vault_dir = &config.vault_dir;

    // Ensure source exists
    let source_secrets = store::load_secrets(vault_dir, source)
        .with_context(|| format!("Failed to load source environment '{}'", source))?;

    if source_secrets.is_empty() {
        bail!("Source environment '{}' is empty or does not exist.", source);
    }

    // Ensure destination does not already exist
    let dest_path = PathBuf::from(vault_dir).join(format!("{}.age", destination));
    if dest_path.exists() {
        bail!(
            "Destination environment '{}' already exists. Use a different name or delete it first.",
            destination
        );
    }

    // Write source secrets into destination
    store::save_secrets(vault_dir, destination, &source_secrets, &config.public_key)
        .with_context(|| format!("Failed to save secrets to destination '{}'", destination))?;

    println!(
        "Environment '{}' cloned to '{}' ({} secret(s) copied).",
        source,
        destination,
        source_secrets.len()
    );

    Ok(())
}
