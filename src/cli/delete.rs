use crate::store::VaultStore;
use crate::config::Config;
use crate::crypto::encrypt::encrypt_store;
use anyhow::{Result, Context};

/// Remove a key from the vault and re-encrypt it.
pub fn handle_delete(config: &Config, key: &str) -> Result<()> {
    let mut store = VaultStore::load(&config.vault_path, &config.key_path)
        .context("Failed to load vault. Is it locked? Run `vaultkeeper unlock` first.")?;

    run_delete(&mut store, key)
        .with_context(|| format!("Failed to delete key `{}`", key))?;

    encrypt_store(&store, &config.vault_path, &config.public_key)
        .context("Failed to re-encrypt vault after deleting key")?;

    println!("Deleted `{}` from vault.", key);
    Ok(())
}

/// Core delete logic, separated for testability.
pub fn run_delete(store: &mut VaultStore, key: &str) -> Result<()> {
    if store.get(key).is_none() {
        anyhow::bail!("Key `{}` not found in vault", key);
    }
    store.remove(key);
    Ok(())
}
