use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::store::Store;
use crate::config::Config;

/// Import secrets from a plaintext .env file into the vault
pub fn run(env_file: &PathBuf, vault: Option<&str>, force: bool) -> Result<()> {
    let config = Config::load()?;
    let vault_name = vault.unwrap_or(&config.default_vault);

    let content = fs::read_to_string(env_file)
        .with_context(|| format!("Failed to read env file: {}", env_file.display()))?;

    let mut store = Store::load(vault_name)?;
    let mut imported = 0;
    let mut skipped = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if store.has(key) && !force {
                eprintln!("Skipping existing key: {} (use --force to overwrite)", key);
                skipped += 1;
                continue;
            }
            store.set(key, value);
            imported += 1;
        }
    }

    store.save(vault_name)?;
    println!(
        "Imported {} secret(s) into vault '{}'{}",
        imported,
        vault_name,
        if skipped > 0 { format!(" ({} skipped)", skipped) } else { String::new() }
    );
    Ok(())
}
