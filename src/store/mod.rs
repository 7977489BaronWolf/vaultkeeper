use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const VAULT_INDEX_FILE: &str = ".vault_index.json";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VaultStore {
    pub entries: HashMap<String, VaultEntry>,
    #[serde(skip)]
    index_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub name: String,
    pub encrypted_file: String,
    pub created_at: String,
}

impl VaultStore {
    pub fn load(vault_dir: &Path) -> Result<Self> {
        let index_path = vault_dir.join(VAULT_INDEX_FILE);
        if !index_path.exists() {
            anyhow::bail!("Vault index not found at {:?}", index_path);
        }
        let data = fs::read_to_string(&index_path)
            .context("Failed to read vault index")?;
        let mut store: VaultStore = serde_json::from_str(&data)
            .context("Failed to parse vault index")?;
        store.index_path = index_path;
        Ok(store)
    }

    pub fn load_or_create(vault_dir: &Path) -> Result<Self> {
        let index_path = vault_dir.join(VAULT_INDEX_FILE);
        if index_path.exists() {
            return Self::load(vault_dir);
        }
        Ok(VaultStore {
            entries: HashMap::new(),
            index_path,
        })
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)
            .context("Failed to serialize vault index")?;
        fs::write(&self.index_path, data)
            .context("Failed to write vault index")?;
        Ok(())
    }

    pub fn insert(&mut self, entry: VaultEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    pub fn list_entries(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.entries.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    pub fn get(&self, name: &str) -> Option<&VaultEntry> {
        self.entries.get(name)
    }
}
