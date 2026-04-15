use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::cli::keygen::generate_keypair;

const DEFAULT_ENV_FILE: &str = ".env";
const DEFAULT_VAULT_DIR: &str = ".vaultkeeper";
const IDENTITY_FILE: &str = ".vaultkeeper/identity.txt";
const RECIPIENT_FILE: &str = ".vaultkeeper/recipient.txt";
const CONFIG_FILE: &str = ".vaultkeeper/config.toml";

pub fn run(force: bool) -> Result<()> {
    let vault_dir = Path::new(DEFAULT_VAULT_DIR);

    if vault_dir.exists() && !force {
        anyhow::bail!(
            "Vault already initialized at '{}'. Use --force to reinitialize.",
            DEFAULT_VAULT_DIR
        );
    }

    fs::create_dir_all(vault_dir)
        .context("Failed to create .vaultkeeper directory")?;

    let (secret_key, public_key) = generate_keypair()
        .context("Failed to generate age keypair")?;

    fs::write(IDENTITY_FILE, format!("{}\n", secret_key))
        .context("Failed to write identity file")?;

    fs::write(RECIPIENT_FILE, format!("{}\n", public_key))
        .context("Failed to write recipient file")?;

    let config_content = format!(
        "# VaultKeeper configuration\nenv_file = \"{}\"\nlocked_file = \"{}.age\"\n",
        DEFAULT_ENV_FILE, DEFAULT_ENV_FILE
    );
    fs::write(CONFIG_FILE, config_content)
        .context("Failed to write config file")?;

    // Ensure .env exists so users have something to lock
    if !Path::new(DEFAULT_ENV_FILE).exists() {
        fs::write(DEFAULT_ENV_FILE, "# Add your secrets here\n")
            .context("Failed to create default .env file")?;
        println!("Created default '{}'.", DEFAULT_ENV_FILE);
    }

    println!("Vault initialized successfully.");
    println!("  Identity : {}", IDENTITY_FILE);
    println!("  Recipient: {}", RECIPIENT_FILE);
    println!("  Config   : {}", CONFIG_FILE);
    println!();
    println!("Keep '{}' secret and back it up securely!", IDENTITY_FILE);

    Ok(())
}
