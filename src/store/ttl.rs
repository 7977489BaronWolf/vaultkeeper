use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlEntry {
    pub key: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TtlStore {
    pub entries: HashMap<String, u64>,
}

impl TtlStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_ttl(&mut self, key: &str, seconds: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.entries.insert(key.to_string(), now + seconds);
    }

    pub fn is_expired(&self, key: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.entries.get(key).map_or(false, |&exp| now >= exp)
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    pub fn expired_keys(&self) -> Vec<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.entries
            .iter()
            .filter(|(_, &exp)| now >= exp)
            .map(|(k, _)| k.clone())
            .collect()
    }

    pub fn ttl_remaining(&self, key: &str) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.entries.get(key).and_then(|&exp| {
            if exp > now { Some(exp - now) } else { None }
        })
    }
}
