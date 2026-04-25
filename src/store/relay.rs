use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A relay maps a secret key to one or more forwarding targets (other keys or namespaces).
/// This allows a single source-of-truth key to propagate its value automatically.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelayRule {
    pub source: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelayStore {
    pub rules: Vec<RelayRule>,
}

impl RelayStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, source: &str, targets: Vec<String>) -> Result<(), String> {
        if source.trim().is_empty() {
            return Err("Source key must not be empty".into());
        }
        if targets.is_empty() {
            return Err("At least one target must be specified".into());
        }
        // Remove existing rule for same source
        self.rules.retain(|r| r.source != source);
        self.rules.push(RelayRule {
            source: source.to_string(),
            targets,
        });
        Ok(())
    }

    pub fn remove_rule(&mut self, source: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.source != source);
        self.rules.len() < before
    }

    pub fn get_targets(&self, source: &str) -> Option<&Vec<String>> {
        self.rules.iter().find(|r| r.source == source).map(|r| &r.targets)
    }

    /// Resolve propagated values given a map of current secrets.
    /// Returns a map of target -> value for all relay rules that have a matching source.
    pub fn resolve(&self, secrets: &HashMap<String, String>) -> HashMap<String, String> {
        let mut propagated = HashMap::new();
        for rule in &self.rules {
            if let Some(value) = secrets.get(&rule.source) {
                for target in &rule.targets {
                    propagated.insert(target.clone(), value.clone());
                }
            }
        }
        propagated
    }

    pub fn list(&self) -> &[RelayRule] {
        &self.rules
    }
}
