use std::path::Path;
use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::store::SecretStore;

pub fn run(config: &Config) -> Result<()> {
    let vault_path = Path::new(&config.vault_path);
    let key_path = Path::new(&config.key_path);

    println!("{}", "VaultKeeper Status".bold().underline());
    println!();

    // Check vault file
    if vault_path.exists() {
        println!(
            "  {} {}",
            "✔".green().bold(),
            format!("Vault file found: {}", config.vault_path).white()
        );
    } else {
        println!(
            "  {} {}",
            "✘".red().bold(),
            format!("Vault file not found: {}", config.vault_path).dimmed()
        );
    }

    // Check key file
    if key_path.exists() {
        println!(
            "  {} {}",
            "✔".green().bold(),
            format!("Key file found: {}", config.key_path).white()
        );
    } else {
        println!(
            "  {} {}",
            "✘".red().bold(),
            format!("Key file not found: {}", config.key_path).dimmed()
        );
    }

    // Check secret count if store is accessible
    if vault_path.exists() && key_path.exists() {
        match SecretStore::load(vault_path) {
            Ok(store) => {
                let count = store.len();
                println!(
                    "  {} {}",
                    "ℹ".cyan().bold(),
                    format!("Secrets stored: {}", count).white()
                );
            }
            Err(_) => {
                println!(
                    "  {} {}",
                    "!".yellow().bold(),
                    "Could not read secret count (vault may be locked)".dimmed()
                );
            }
        }
    }

    println!();
    Ok(())
}
