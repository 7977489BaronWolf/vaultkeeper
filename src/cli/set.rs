use crate::store::VaultStore;
use crate::config::Config;
use crate::crypto::encrypt::encrypt_store;
use anyhow::{Result, Context};

/// Set (or update) a key-value pair in the vault and re-encrypt it.
pub fn handle_set(config: &Config, key: &str, value: &str) -> Result<()> {
    if key.is_empty() {
        anyhow::bail!("Key must not be empty");
    }
    if key.contains('=') {
        anyhow::bail!("Key must not contain '=' character");
    }

    let mut store = VaultStore::load(&config.vault_path, &config.key_path)
        .context("Failed to load vault. Is it locked? Run `vaultkeeper unlock` first.")?;

    let is_update = store.get(key).is_some();
    store.set(key, value);

    encrypt_store(&store, &config.vault_path, &config.public_key)
        .context("Failed to re-encrypt vault after setting key")?;

    if is_update {
        println!("Updated `{}` in vault.", key);
    } else {
        println!("Set `{}` in vault.", key);
    }

    Ok(())
}

/// Core logic for set, separated for testability.
pub fn run_set(store: &mut VaultStore, key: &str, value: &str) -> Result<String> {
    if key.is_empty() {
        anyhow::bail!("Key must not be empty");
    }
    if key.contains('=') {
        anyhow::bail!("Key '{}' must not contain '=' character", key);
    }

    let is_update = store.get(key).is_some();
    store.set(key, value);

    if is_update {
        Ok(format!("Updated `{}`", key))
    } else {
        Ok(format!("Set `{}`", key))
    }
}
