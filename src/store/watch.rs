use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatchEntry {
    pub path: PathBuf,
    pub last_modified: u64,
    pub size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct WatchIndex {
    pub entries: HashMap<String, WatchEntry>,
}

impl WatchIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track(&mut self, name: &str, path: &Path) -> std::io::Result<()> {
        let meta = std::fs::metadata(path)?;
        let last_modified = meta
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.entries.insert(
            name.to_string(),
            WatchEntry {
                path: path.to_path_buf(),
                last_modified,
                size: meta.len(),
            },
        );
        Ok(())
    }

    pub fn is_modified(&self, name: &str, path: &Path) -> std::io::Result<bool> {
        let meta = std::fs::metadata(path)?;
        let current_modified = meta
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(match self.entries.get(name) {
            Some(entry) => entry.last_modified != current_modified || entry.size != meta.len(),
            None => true,
        })
    }

    pub fn untrack(&mut self, name: &str) {
        self.entries.remove(name);
    }

    pub fn tracked_names(&self) -> Vec<&String> {
        self.entries.keys().collect()
    }
}
