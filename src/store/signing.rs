use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignedEntry {
    pub key: String,
    pub signature: String,
    pub signer: String,
    pub signed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SigningStore {
    pub entries: HashMap<String, SignedEntry>,
}

impl SigningStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sign(&mut self, key: &str, value: &str, signer: &str) -> SignedEntry {
        let signature = compute_signature(key, value, signer);
        let entry = SignedEntry {
            key: key.to_string(),
            signature,
            signer: signer.to_string(),
            signed_at: current_timestamp(),
        };
        self.entries.insert(key.to_string(), entry.clone());
        entry
    }

    pub fn verify(&self, key: &str, value: &str) -> bool {
        match self.entries.get(key) {
            Some(entry) => {
                let expected = compute_signature(key, value, &entry.signer);
                entry.signature == expected
            }
            None => false,
        }
    }

    pub fn get_signature(&self, key: &str) -> Option<&SignedEntry> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn list_signed(&self) -> Vec<&SignedEntry> {
        self.entries.values().collect()
    }
}

fn compute_signature(key: &str, value: &str, signer: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    format!("{key}:{value}:{signer}").hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
