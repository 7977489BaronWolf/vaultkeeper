use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct VersionedEntry {
    pub version: u32,
    pub value: String,
    pub created_at: u64,
}

#[derive(Debug, Default)]
pub struct VersionStore {
    entries: HashMap<String, Vec<VersionedEntry>>,
}

impl VersionStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn push(&mut self, key: &str, value: &str, timestamp: u64) {
        let versions = self.entries.entry(key.to_string()).or_default();
        let next_version = versions.last().map(|v| v.version + 1).unwrap_or(1);
        versions.push(VersionedEntry {
            version: next_version,
            value: value.to_string(),
            created_at: timestamp,
        });
    }

    pub fn get_version(&self, key: &str, version: u32) -> Option<&VersionedEntry> {
        self.entries
            .get(key)?
            .iter()
            .find(|e| e.version == version)
    }

    pub fn latest(&self, key: &str) -> Option<&VersionedEntry> {
        self.entries.get(key)?.last()
    }

    pub fn list_versions(&self, key: &str) -> Vec<&VersionedEntry> {
        self.entries
            .get(key)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn rollback(&mut self, key: &str, version: u32) -> bool {
        if let Some(versions) = self.entries.get(key) {
            if let Some(entry) = versions.iter().find(|e| e.version == version) {
                let value = entry.value.clone();
                let ts = entry.created_at;
                self.push(key, &value, ts);
                return true;
            }
        }
        false
    }

    pub fn count(&self, key: &str) -> usize {
        self.entries.get(key).map(|v| v.len()).unwrap_or(0)
    }
}
