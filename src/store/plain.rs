//! Plain (unencrypted) storage helpers used in tests and intermediate operations.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Save secrets as a plain `.env`-style file (used in tests).
pub fn save_secrets_plain(
    vault_dir: &str,
    env_name: &str,
    secrets: &HashMap<String, String>,
) -> Result<()> {
    fs::create_dir_all(vault_dir)
        .with_context(|| format!("Failed to create vault directory: {}", vault_dir))?;

    let path = PathBuf::from(vault_dir).join(format!("{}.age", env_name));
    let mut lines: Vec<String> = secrets
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    lines.sort();
    let content = lines.join("\n") + "\n";
    fs::write(&path, content)
        .with_context(|| format!("Failed to write plain secrets to {:?}", path))?;
    Ok(())
}

/// Load secrets from a plain `.env`-style file (used in tests).
pub fn load_secrets_plain(
    vault_dir: &str,
    env_name: &str,
) -> Result<HashMap<String, String>> {
    let path = PathBuf::from(vault_dir).join(format!("{}.age", env_name));
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read plain secrets from {:?}", path))?;

    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            map.insert(key.trim().to_string(), val.trim().to_string());
        }
    }
    Ok(map)
}
