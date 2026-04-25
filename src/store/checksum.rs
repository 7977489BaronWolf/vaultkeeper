use std::collections::HashMap;
use sha2::{Digest, Sha256};

/// Compute a SHA-256 checksum for a map of key-value secrets.
pub fn compute_checksum(secrets: &HashMap<String, String>) -> String {
    let mut pairs: Vec<(&String, &String)> = secrets.iter().collect();
    pairs.sort_by_key(|(k, _)| *k);

    let mut hasher = Sha256::new();
    for (k, v) in pairs {
        hasher.update(k.as_bytes());
        hasher.update(b"=");
        hasher.update(v.as_bytes());
        hasher.update(b"\n");
    }

    format!("{:x}", hasher.finalize())
}

/// Verify that the provided checksum matches the computed one.
pub fn verify_checksum(secrets: &HashMap<String, String>, expected: &str) -> bool {
    compute_checksum(secrets) == expected
}

/// Compute a checksum for a single key-value pair.
pub fn compute_entry_checksum(key: &str, value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(b"=");
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
#[path = "checksum_tests.rs"]
mod tests;
