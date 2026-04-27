use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteConfig {
    pub name: String,
    pub url: String,
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RemoteStore {
    remotes: HashMap<String, RemoteConfig>,
}

impl RemoteStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, config: RemoteConfig) -> Result<(), String> {
        if self.remotes.contains_key(&config.name) {
            return Err(format!("Remote '{}' already exists", config.name));
        }
        self.remotes.insert(config.name.clone(), config);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<RemoteConfig, String> {
        self.remotes
            .remove(name)
            .ok_or_else(|| format!("Remote '{}' not found", name))
    }

    pub fn get(&self, name: &str) -> Option<&RemoteConfig> {
        self.remotes.get(name)
    }

    pub fn list(&self) -> Vec<&RemoteConfig> {
        let mut remotes: Vec<&RemoteConfig> = self.remotes.values().collect();
        remotes.sort_by(|a, b| a.name.cmp(&b.name));
        remotes
    }

    pub fn update_token(&mut self, name: &str, token: Option<String>) -> Result<(), String> {
        let remote = self
            .remotes
            .get_mut(name)
            .ok_or_else(|| format!("Remote '{}' not found", name))?;
        remote.auth_token = token;
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.remotes.is_empty()
    }

    pub fn count(&self) -> usize {
        self.remotes.len()
    }
}
