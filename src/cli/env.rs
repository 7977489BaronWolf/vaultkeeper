use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::crypto::decrypt::decrypt_file;
use crate::store::VaultStore;

/// Parse a decrypted env file into a key-value map.
pub fn parse_env_contents(contents: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').to_string();
            if !key.is_empty() {
                map.insert(key, value);
            }
        }
    }
    map
}

/// Load and decrypt env variables for a given vault entry.
pub fn load_env_vars(
    config: &Config,
    store: &VaultStore,
    env_name: &str,
) -> Result<HashMap<String, String>> {
    let entry = store
        .get(env_name)
        .with_context(|| format!("No vault entry found for '{}'", env_name))?;

    let encrypted_path = PathBuf::from(&entry.encrypted_path);
    if !encrypted_path.exists() {
        anyhow::bail!(
            "Encrypted file not found: {}",
            encrypted_path.display()
        );
    }

    let identity_path = config.identity_path();
    let decrypted = decrypt_file(&encrypted_path, &identity_path)
        .with_context(|| format!("Failed to decrypt '{}'", env_name))?;

    let contents = String::from_utf8(decrypted)
        .context("Decrypted content is not valid UTF-8")?;

    Ok(parse_env_contents(&contents))
}

/// Format a key-value map back into env file contents.
pub fn format_env_contents(vars: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = vars
        .iter()
        .map(|(k, v)| {
            if v.contains(' ') || v.contains('"') {
                format!("{}=\"{}\"", k, v.replace('"', "\\\""))
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect();
    lines.sort();
    lines.join("\n") + "\n"
}
