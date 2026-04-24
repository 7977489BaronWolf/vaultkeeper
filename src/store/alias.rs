use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AliasStore {
    pub aliases: HashMap<String, String>,
}

impl AliasStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, alias: &str, target: &str) -> Result<(), String> {
        if alias.trim().is_empty() {
            return Err("Alias name cannot be empty".to_string());
        }
        if target.trim().is_empty() {
            return Err("Alias target cannot be empty".to_string());
        }
        if alias == target {
            return Err("Alias cannot point to itself".to_string());
        }
        self.aliases.insert(alias.to_string(), target.to_string());
        Ok(())
    }

    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }

    pub fn remove(&mut self, alias: &str) -> Result<(), String> {
        if self.aliases.remove(alias).is_none() {
            return Err(format!("Alias '{}' not found", alias));
        }
        Ok(())
    }

    pub fn resolve(&self, key: &str) -> &str {
        self.aliases.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut pairs: Vec<_> = self.aliases.iter().collect();
        pairs.sort_by_key(|(k, _)| k.as_str());
        pairs
    }

    pub fn has_cycle(&self, alias: &str, target: &str) -> bool {
        let mut current = target;
        let mut visited = vec![alias];
        loop {
            match self.aliases.get(current) {
                Some(next) => {
                    if visited.contains(&next.as_str()) {
                        return true;
                    }
                    visited.push(current);
                    current = next.as_str();
                }
                None => return false,
            }
        }
    }
}
