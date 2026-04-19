use std::collections::HashMap;

/// A pinned secret is one marked as protected from bulk operations
/// like wipe, rotate, or import overwrite.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PinStore {
    pub pinned: HashMap<String, bool>,
}

impl PinStore {
    pub fn new() -> Self {
        Self {
            pinned: HashMap::new(),
        }
    }

    pub fn pin(&mut self, key: &str) {
        self.pinned.insert(key.to_string(), true);
    }

    pub fn unpin(&mut self, key: &str) {
        self.pinned.remove(key);
    }

    pub fn is_pinned(&self, key: &str) -> bool {
        self.pinned.get(key).copied().unwrap_or(false)
    }

    pub fn list_pinned(&self) -> Vec<&String> {
        self.pinned
            .iter()
            .filter(|(_, &v)| v)
            .map(|(k, _)| k)
            .collect()
    }

    pub fn filter_unpinned<'a>(&self, keys: &[&'a str]) -> Vec<&'a str> {
        keys.iter()
            .copied()
            .filter(|k| !self.is_pinned(k))
            .collect()
    }
}
