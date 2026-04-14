pub mod encrypt;
pub mod decrypt;

use std::path::Path;
use anyhow::Result;

/// Reads a public key from a file or returns it as-is if it looks like a raw key.
pub fn resolve_recipient(recipient: &str) -> Result<String> {
    let path = Path::new(recipient);
    if path.exists() {
        let contents = std::fs::read_to_string(path)?;
        let key = contents
            .lines()
            .find(|line| line.starts_with("age") && !line.starts_with("# "))
            .ok_or_else(|| anyhow::anyhow!("No valid public key found in file: {}", recipient))?
            .to_string();
        Ok(key)
    } else {
        Ok(recipient.to_string())
    }
}
