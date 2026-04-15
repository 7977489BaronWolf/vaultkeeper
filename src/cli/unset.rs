use crate::config::Config;
use crate::store::Store;
use anyhow::{Context, Result};

/// Remove one or more keys from the decrypted env store,
/// then re-encrypt the vault in place.
pub fn run(keys: &[String], config: &Config) -> Result<()> {
    if keys.is_empty() {
        anyhow::bail!("No keys provided. Usage: vaultkeeper unset KEY [KEY…]");
    }

    let identity_path = config.identity_path();
    let vault_path = config.vault_path();

    let mut store = Store::load_encrypted(&vault_path, &identity_path)
        .context("Failed to load vault — is it locked?")?;

    let mut removed: Vec<&str> = Vec::new();
    let mut missing: Vec<&str> = Vec::new();

    for key in keys {
        if store.remove(key.as_str()) {
            removed.push(key.as_str());
        } else {
            missing.push(key.as_str());
        }
    }

    if removed.is_empty() {
        eprintln!("None of the specified keys were found in the vault.");
        for k in &missing {
            eprintln!("  missing: {}", k);
        }
        anyhow::bail!("Nothing removed.");
    }

    store
        .save_encrypted(&vault_path, &config.recipient_path())
        .context("Failed to re-encrypt vault after removal")?;

    for k in &removed {
        println!("Removed: {}", k);
    }

    for k in &missing {
        eprintln!("Warning: key not found (skipped): {}", k);
    }

    Ok(())
}
