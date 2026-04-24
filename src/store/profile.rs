use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub env_overrides: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            env_overrides: HashMap::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn set_override(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env_overrides.insert(key.into(), value.into());
    }

    pub fn remove_override(&mut self, key: &str) -> bool {
        self.env_overrides.remove(key).is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileStore {
    pub profiles: HashMap<String, Profile>,
    pub active: Option<String>,
}

impl ProfileStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, profile: Profile) -> Result<(), String> {
        if self.profiles.contains_key(&profile.name) {
            return Err(format!("profile '{}' already exists", profile.name));
        }
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.active.as_deref() == Some(name) {
            return Err(format!("cannot remove active profile '{}'", name));
        }
        self.profiles.remove(name).ok_or_else(|| format!("profile '{}' not found", name))?;
        Ok(())
    }

    pub fn activate(&mut self, name: &str) -> Result<(), String> {
        if !self.profiles.contains_key(name) {
            return Err(format!("profile '{}' not found", name));
        }
        self.active = Some(name.to_string());
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.active = None;
    }

    pub fn get_active(&self) -> Option<&Profile> {
        self.active.as_deref().and_then(|n| self.profiles.get(n))
    }

    pub fn list(&self) -> Vec<&Profile> {
        let mut profiles: Vec<&Profile> = self.profiles.values().collect();
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        profiles
    }
}
