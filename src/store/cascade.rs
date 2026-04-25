use std::collections::HashMap;

/// Represents a cascade resolution order for secret lookups.
/// Secrets are resolved from highest to lowest priority namespace/profile.
#[derive(Debug, Clone)]
pub struct CascadeChain {
    pub layers: Vec<String>,
}

impl CascadeChain {
    pub fn new(layers: Vec<String>) -> Self {
        Self { layers }
    }

    /// Resolve a key by walking layers from first (highest priority) to last.
    pub fn resolve<'a>(
        &self,
        key: &str,
        stores: &'a HashMap<String, HashMap<String, String>>,
    ) -> Option<(&'a str, &'a str)> {
        for layer in &self.layers {
            if let Some(store) = stores.get(layer) {
                if let Some(value) = store.get(key) {
                    return Some((layer.as_str(), value.as_str()));
                }
            }
        }
        None
    }

    /// Merge all layers into a single map, higher-priority layers win.
    pub fn flatten(
        &self,
        stores: &HashMap<String, HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        // Iterate in reverse so higher-priority layers overwrite lower ones.
        for layer in self.layers.iter().rev() {
            if let Some(store) = stores.get(layer) {
                for (k, v) in store {
                    result.insert(k.clone(), v.clone());
                }
            }
        }
        result
    }

    /// List all keys available across the chain (deduplicated).
    pub fn all_keys(&self, stores: &HashMap<String, HashMap<String, String>>) -> Vec<String> {
        let mut keys: Vec<String> = self.flatten(stores).into_keys().collect();
        keys.sort();
        keys
    }
}
