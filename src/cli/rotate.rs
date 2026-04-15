use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::config::Config;
use crate::crypto::decrypt::decrypt_file;
use crate::crypto::encrypt::encrypt_file;
use crate::store::VaultStore;

/// Rotate the encryption key for a vault by re-encrypting with a new key.
/// The old key is used to decrypt, and the new key is used to re-encrypt.
pub fn handle_rotate(
    vault_name: &str,
    old_key_path: &PathBuf,
    new_key_path: &PathBuf,
    config: &Config,
) -> Result<()> {
    let store = VaultStore::load(&config.store_path)
        .context("Failed to load vault store")?;

    if !store.vault_exists(vault_name) {
        anyhow::bail!("Vault '{}' does not exist", vault_name);
    }

    let vault_meta = store
        .get_vault(vault_name)
        .context("Failed to retrieve vault metadata")?;

    if !vault_meta.is_locked {
        anyhow::bail!(
            "Vault '{}' must be locked before rotating keys. Run `vaultkeeper lock {}` first.",
            vault_name,
            vault_name
        );
    }

    let encrypted_path = &vault_meta.encrypted_path;

    // Decrypt to a temporary buffer using the old key
    let plaintext = decrypt_file(encrypted_path, old_key_path)
        .context("Failed to decrypt vault with old key. Is the key correct?")?;

    // Re-encrypt using the new key
    encrypt_file(&plaintext, encrypted_path, new_key_path)
        .context("Failed to re-encrypt vault with new key")?;

    println!(
        "✓ Key rotated successfully for vault '{}'",
        vault_name
    );
    println!("  Encrypted path : {}", encrypted_path.display());
    println!("  New key        : {}", new_key_path.display());

    Ok(())
}
