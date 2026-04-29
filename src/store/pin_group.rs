use std::collections::{HashMap, HashSet};

/// A named group of pinned keys that must all be present and unchanged.
#[derive(Debug, Clone, PartialEq)]
pub struct PinGroup {
    pub name: String,
    pub keys: HashSet<String>,
}

impl PinGroup {
    pub fn new(name: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            name: name.into(),
            keys: keys.into_iter().collect(),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    pub fn add_key(&mut self, key: impl Into<String>) {
        self.keys.insert(key.into());
    }

    pub fn remove_key(&mut self, key: &str) -> bool {
        self.keys.remove(key)
    }
}

/// Registry of all pin groups in a vault.
#[derive(Debug, Default, Clone)]
pub struct PinGroupRegistry {
    groups: HashMap<String, PinGroup>,
}

impl PinGroupRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_group(&mut self, group: PinGroup) {
        self.groups.insert(group.name.clone(), group);
    }

    pub fn remove_group(&mut self, name: &str) -> Option<PinGroup> {
        self.groups.remove(name)
    }

    pub fn get_group(&self, name: &str) -> Option<&PinGroup> {
        self.groups.get(name)
    }

    pub fn list_groups(&self) -> Vec<&PinGroup> {
        let mut groups: Vec<&PinGroup> = self.groups.values().collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        groups
    }

    /// Returns all group names that contain the given key.
    pub fn groups_for_key(&self, key: &str) -> Vec<&str> {
        self.groups
            .values()
            .filter(|g| g.contains(key))
            .map(|g| g.name.as_str())
            .collect()
    }

    /// Validate that all keys in every group exist in the provided key set.
    pub fn validate(&self, existing_keys: &HashSet<String>) -> Vec<String> {
        let mut missing = Vec::new();
        for group in self.groups.values() {
            for key in &group.keys {
                if !existing_keys.contains(key) {
                    missing.push(format!("[{}] {}", group.name, key));
                }
            }
        }
        missing.sort();
        missing
    }
}
