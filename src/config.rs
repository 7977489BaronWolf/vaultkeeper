use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vault_dir: PathBuf,
    pub default_env: String,
}

impl Config {
    pub fn load(vault_dir: &Path) -> Result<Self> {
        let config_path = vault_dir.join(".vaultkeeper").join("config.toml");
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self {
                vault_dir: vault_dir.to_path_buf(),
                default_env: "development".to_string(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = self.vault_dir.join(".vaultkeeper");
        std::fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    pub fn plain_path(&self, env: &str) -> PathBuf {
        self.vault_dir.join(".vaultkeeper").join("plain").join(format!("{}.env", env))
    }

    pub fn encrypted_path(&self, env: &str) -> PathBuf {
        self.vault_dir.join(format!("{}.env.age", env))
    }

    pub fn keys_dir(&self) -> PathBuf {
        self.vault_dir.join(".vaultkeeper").join("keys")
    }

    pub fn snapshots_dir(&self) -> PathBuf {
        self.vault_dir.join(".vaultkeeper").join("snapshots")
    }
}
