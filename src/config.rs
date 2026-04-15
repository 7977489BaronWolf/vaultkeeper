use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = ".vaultkeeper/config.toml";
const IDENTITY_PATH: &str = ".vaultkeeper/identity.txt";
const RECIPIENT_PATH: &str = ".vaultkeeper/recipient.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub env_file: String,
    pub locked_file: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            env_file: ".env".to_string(),
            locked_file: ".env.age".to_string(),
        }
    }
}

impl VaultConfig {
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string(CONFIG_PATH)
            .context("Failed to read config. Have you run `vaultkeeper init`?")?;
        toml::from_str(&content).context("Failed to parse config.toml")
    }

    pub fn identity() -> Result<String> {
        let raw = fs::read_to_string(IDENTITY_PATH)
            .context("Identity file not found. Have you run `vaultkeeper init`?")?;
        Ok(raw.trim().to_string())
    }

    pub fn recipient() -> Result<String> {
        let raw = fs::read_to_string(RECIPIENT_PATH)
            .context("Recipient file not found. Have you run `vaultkeeper init`?")?;
        Ok(raw.trim().to_string())
    }

    pub fn is_initialized() -> bool {
        Path::new(CONFIG_PATH).exists()
            && Path::new(IDENTITY_PATH).exists()
            && Path::new(RECIPIENT_PATH).exists()
    }
}
