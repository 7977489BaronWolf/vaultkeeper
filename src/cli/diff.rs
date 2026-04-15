use crate::config::Config;
use crate::crypto::decrypt::decrypt_file;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

/// Parse env file content into a key-value map
fn parse_env_content(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}

/// Show diff between two vaults or between a vault and plaintext env file
pub fn run_diff(config: &Config, target_a: &str, target_b: Option<&str>) -> Result<()> {
    let key_path = config.key_path();

    let content_a = if target_a.ends_with(".age") {
        decrypt_file(target_a, &key_path).context(format!("Failed to decrypt {}", target_a))?
    } else {
        fs::read_to_string(target_a).context(format!("Failed to read {}", target_a))?
    };

    let content_b = match target_b {
        Some(b) => {
            if b.ends_with(".age") {
                decrypt_file(b, &key_path).context(format!("Failed to decrypt {}", b))?
            } else {
                fs::read_to_string(b).context(format!("Failed to read {}", b))?
            }
        }
        None => {
            let plain = config.env_file();
            fs::read_to_string(&plain)
                .context(format!("Failed to read plaintext env: {}", plain))?
        }
    };

    let map_a = parse_env_content(&content_a);
    let map_b = parse_env_content(&content_b);

    let mut all_keys: Vec<&String> = map_a.keys().chain(map_b.keys()).collect();
    all_keys.sort();
    all_keys.dedup();

    let mut has_diff = false;

    for key in all_keys {
        match (map_a.get(key), map_b.get(key)) {
            (Some(va), Some(vb)) if va != vb => {
                println!("~ {} : {} -> {}", key, va, vb);
                has_diff = true;
            }
            (Some(va), None) => {
                println!("- {} = {}", key, va);
                has_diff = true;
            }
            (None, Some(vb)) => {
                println!("+ {} = {}", key, vb);
                has_diff = true;
            }
            _ => {}
        }
    }

    if !has_diff {
        println!("No differences found.");
    }

    Ok(())
}
