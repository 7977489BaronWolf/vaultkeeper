use std::collections::VecDeque;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

const MAX_HISTORY: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub key: String,
    pub env: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: VecDeque<HistoryEntry>,
}

impl History {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn push(&mut self, action: &str, key: &str, env: &str) {
        if self.entries.len() >= MAX_HISTORY {
            self.entries.pop_front();
        }
        self.entries.push_back(HistoryEntry {
            timestamp: Utc::now(),
            action: action.to_string(),
            key: key.to_string(),
            env: env.to_string(),
        });
    }

    pub fn for_env<'a>(&'a self, env: &str) -> impl Iterator<Item = &'a HistoryEntry> {
        self.entries.iter().filter(move |e| e.env == env)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
