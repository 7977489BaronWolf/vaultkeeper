use crate::config::Config;
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Display information about the current vaultkeeper identity:
/// active key fingerprint, config path, and vault store location.
pub fn run(config_path: Option<PathBuf>) -> Result<()> {
    let cfg_path = config_path
        .unwrap_or_else(Config::default_path);

    let config = Config::load(&cfg_path)
        .with_context(|| format!("Failed to load config from {}", cfg_path.display()))?;

    println!("vaultkeeper identity");
    println!("  config     : {}", cfg_path.display());
    println!(
        "  store      : {}",
        config.store_path().display()
    );

    match config.public_key() {
        Some(key) => {
            let fingerprint = fingerprint_key(key);
            println!("  public key : {}", key);
            println!("  fingerprint: {}", fingerprint);
        }
        None => {
            println!("  public key : (none — run `vaultkeeper keygen` to create a key)");
        }
    }

    Ok(())
}

/// Produce a short human-readable fingerprint from an age public key string.
/// Takes the last 8 characters of the key as a simple display aid.
fn fingerprint_key(key: &str) -> String {
    let trimmed = key.trim();
    if trimmed.len() >= 8 {
        format!("...{}", &trimmed[trimmed.len() - 8..])
    } else {
        trimmed.to_string()
    }
}
