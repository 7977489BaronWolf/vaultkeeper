use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::store::Store;
use crate::config::Config;

/// Export secrets from the vault to a plaintext .env file
pub fn run(output: &PathBuf, vault: Option<&str>, overwrite: bool) -> Result<()> {
    let config = Config::load()?;
    let vault_name = vault.unwrap_or(&config.default_vault);

    if output.exists() && !overwrite {
        anyhow::bail!(
            "Output file '{}' already exists. Use --overwrite to replace it.",
            output.display()
        );
    }

    let store = Store::load(vault_name)?;
    let secrets = store.all();

    if secrets.is_empty() {
        println!("Vault '{}' is empty, nothing to export.", vault_name);
        return Ok(());
    }

    let mut lines = vec![
        format!("# Exported from vaultkeeper vault: {}", vault_name),
        format!("# Generated at: {}", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")),
        String::new(),
    ];

    let mut keys: Vec<&String> = secrets.keys().collect();
    keys.sort();
    for key in keys {
        let value = &secrets[key];
        let needs_quotes = value.contains(' ') || value.contains('#');
        if needs_quotes {
            lines.push(format!("{}=\"{}\"", key, value));
        } else {
            lines.push(format!("{}={}", key, value));
        }
    }

    let content = lines.join("\n") + "\n";
    fs::write(output, &content)
        .with_context(|| format!("Failed to write to '{}'", output.display()))?;

    println!(
        "Exported {} secret(s) from vault '{}' to '{}'",
        secrets.len(),
        vault_name,
        output.display()
    );
    Ok(())
}
