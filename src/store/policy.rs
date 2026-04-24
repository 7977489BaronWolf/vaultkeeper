use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyRule {
    DenyKeys(Vec<String>),
    RequirePrefix(String),
    MaxValueLength(usize),
    AllowedNamespaces(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Policy {
    pub name: String,
    pub rules: Vec<PolicyRule>,
}

impl Policy {
    pub fn new(name: &str) -> Self {
        Policy {
            name: name.to_string(),
            rules: vec![],
        }
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    pub fn evaluate(&self, key: &str, value: &str, namespace: &str) -> Result<(), String> {
        for rule in &self.rules {
            match rule {
                PolicyRule::DenyKeys(denied) => {
                    if denied.iter().any(|d| d == key) {
                        return Err(format!("Key '{}' is denied by policy '{}'", key, self.name));
                    }
                }
                PolicyRule::RequirePrefix(prefix) => {
                    if !key.starts_with(prefix.as_str()) {
                        return Err(format!(
                            "Key '{}' must start with '{}' per policy '{}'",
                            key, prefix, self.name
                        ));
                    }
                }
                PolicyRule::MaxValueLength(max) => {
                    if value.len() > *max {
                        return Err(format!(
                            "Value for '{}' exceeds max length {} per policy '{}'",
                            key, max, self.name
                        ));
                    }
                }
                PolicyRule::AllowedNamespaces(allowed) => {
                    if !allowed.iter().any(|n| n == namespace) {
                        return Err(format!(
                            "Namespace '{}' not allowed by policy '{}'",
                            namespace, self.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyStore {
    pub policies: HashMap<String, Policy>,
}

impl PolicyStore {
    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    pub fn remove_policy(&mut self, name: &str) -> bool {
        self.policies.remove(name).is_some()
    }

    pub fn get_policy(&self, name: &str) -> Option<&Policy> {
        self.policies.get(name)
    }

    pub fn evaluate_all(&self, key: &str, value: &str, namespace: &str) -> Vec<String> {
        self.policies
            .values()
            .filter_map(|p| p.evaluate(key, value, namespace).err())
            .collect()
    }

    pub fn list(&self) -> Vec<&Policy> {
        let mut policies: Vec<&Policy> = self.policies.values().collect();
        policies.sort_by(|a, b| a.name.cmp(&b.name));
        policies
    }
}
