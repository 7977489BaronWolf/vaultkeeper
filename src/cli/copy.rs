use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::Config;
use crate::crypto::decrypt::decrypt_file;
use crate::crypto::encrypt::encrypt_file;
use crate::store::SecretStore;

/// Copy a secret value from one key to another within the same vault.
pub fn run_copy(
    from_key: &str,
    to_key: &str,
    env: Option<&str>,
    vault_dir: &PathBuf,
) -> Result<()> {
    let config = Config::load(vault_dir)?;
    let env_name = env.unwrap_or(&config.default_env);

    let identity_path = vault_dir.join(&config.identity_file);
    let vault_path = vault_dir.join(format!("{}.age", env_name));

    if !vault_path.exists() {
        anyhow::bail!(
            "Vault '{}' does not exist. Run `vaultkeeper init` first.",
            env_name
        );
    }

    let plaintext = decrypt_file(&vault_path, &identity_path)
        .context("Failed to decrypt vault")?;

    let mut store = SecretStore::from_str(&plaintext)
        .context("Failed to parse vault contents")?;

    let value = store
        .get(from_key)
        .with_context(|| format!("Key '{}' not found in vault '{}'", from_key, env_name))?;

    let value_owned = value.to_string();

    if store.get(to_key).is_some() {
        anyhow::bail!(
            "Key '{}' already exists in vault '{}'. Use `set` to overwrite.",
            to_key,
            env_name
        );
    }

    store.set(to_key, &value_owned);

    let recipient_path = vault_dir.join(&config.recipient_file);
    let updated = store.to_string();

    encrypt_file(updated.as_bytes(), &vault_path, &recipient_path)
        .context("Failed to re-encrypt vault after copy")?;

    println!(
        "Copied '{}' -> '{}' in vault '{}'",
        from_key, to_key, env_name
    );

    Ok(())
}
