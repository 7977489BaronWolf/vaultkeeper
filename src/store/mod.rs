pub mod rename;
pub mod plain;

pub use plain::{load_secrets_plain, save_secrets_plain};

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// List all environment names stored in the vault directory.
pub fn list_envs(vault_dir: &str) -> Result<Vec<String>> {
    let path = PathBuf::from(vault_dir);
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut envs = vec![];
    for entry in fs::read_dir(&path).context("Failed to read vault directory")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.ends_with(".age") {
            envs.push(name.trim_end_matches(".age").to_string());
        }
    }
    envs.sort();
    Ok(envs)
}

/// Load secrets for a given environment (plain fallback for non-encrypted dev use).
pub fn load_secrets(
    vault_dir: &str,
    env_name: &str,
) -> Result<HashMap<String, String>> {
    plain::load_secrets_plain(vault_dir, env_name)
}

/// Save secrets for a given environment.
pub fn save_secrets(
    vault_dir: &str,
    env_name: &str,
    secrets: &HashMap<String, String>,
    _public_key: &str,
) -> Result<()> {
    plain::save_secrets_plain(vault_dir, env_name, secrets)
}

/// Delete an environment file from the vault.
pub fn delete_env(vault_dir: &str, env_name: &str) -> Result<()> {
    let path = PathBuf::from(vault_dir).join(format!("{}.age", env_name));
    fs::remove_file(&path)
        .with_context(|| format!("Failed to delete environment file: {:?}", path))?;
    Ok(())
}

/// Check whether an environment exists in the vault.
pub fn env_exists(vault_dir: &str, env_name: &str) -> bool {
    PathBuf::from(vault_dir)
        .join(format!("{}.age", env_name))
        .exists()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
