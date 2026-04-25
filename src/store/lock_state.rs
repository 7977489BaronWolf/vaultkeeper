use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents whether a secret is locked (read-only) or unlocked
#[derive(Debug, Clone, PartialEq)]
pub enum LockStatus {
    Locked,
    Unlocked,
}

/// Stores lock state metadata per secret key
#[derive(Debug, Clone)]
pub struct LockEntry {
    pub status: LockStatus,
    pub locked_at: Option<u64>,
    pub reason: Option<String>,
}

impl LockEntry {
    pub fn locked(reason: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        LockEntry {
            status: LockStatus::Locked,
            locked_at: Some(now),
            reason,
        }
    }

    pub fn unlocked() -> Self {
        LockEntry {
            status: LockStatus::Unlocked,
            locked_at: None,
            reason: None,
        }
    }

    pub fn is_locked(&self) -> bool {
        self.status == LockStatus::Locked
    }
}

/// In-memory registry of per-key lock states
#[derive(Debug, Default)]
pub struct LockStateStore {
    entries: HashMap<String, LockEntry>,
}

impl LockStateStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock_key(&mut self, key: &str, reason: Option<String>) {
        self.entries.insert(key.to_string(), LockEntry::locked(reason));
    }

    pub fn unlock_key(&mut self, key: &str) {
        self.entries.insert(key.to_string(), LockEntry::unlocked());
    }

    pub fn is_locked(&self, key: &str) -> bool {
        self.entries
            .get(key)
            .map(|e| e.is_locked())
            .unwrap_or(false)
    }

    pub fn get_entry(&self, key: &str) -> Option<&LockEntry> {
        self.entries.get(key)
    }

    pub fn all_locked_keys(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, e)| e.is_locked())
            .map(|(k, _)| k.as_str())
            .collect()
    }

    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }
}
