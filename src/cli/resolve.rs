use crate::config::Config;
use crate::store::resolve::{detect_cycles, resolve_references};
use crate::store::Store;
use anyhow::{Context, Result};

/// Resolves all variable references in the current vault and prints the result.
pub fn run_resolve(config: &Config, show_cycles_only: bool) -> Result<()> {
    let store = Store::load(&config.vault_path).context("Failed to load vault")?;
    let secrets = store.all_plain();

    if let Err(e) = detect_cycles(&secrets) {
        eprintln!("Warning: {}", e);
        if show_cycles_only {
            return Ok(());
        }
    }

    if show_cycles_only {
        println!("No circular references detected.");
        return Ok(());
    }

    let resolved = resolve_references(&secrets);
    let mut keys: Vec<&String> = resolved.keys().collect();
    keys.sort();

    for key in keys {
        let original = secrets.get(key).map(String::as_str).unwrap_or("");
        let value = &resolved[key];
        if original != value {
            println!("{}={} (resolved from: {})", key, value, original);
        } else {
            println!("{}={}", key, value);
        }
    }
    Ok(())
}

/// Returns the resolved value of a single key.
pub fn resolve_key(config: &Config, key: &str) -> Result<String> {
    let store = Store::load(&config.vault_path).context("Failed to load vault")?;
    let secrets = store.all_plain();

    detect_cycles(&secrets).map_err(|e| anyhow::anyhow!(e))?;

    let resolved = resolve_references(&secrets);
    resolved
        .get(key)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Key '{}' not found", key))
}
