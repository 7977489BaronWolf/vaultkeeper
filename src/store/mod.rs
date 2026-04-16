pub mod audit;
pub mod history;
pub mod merge;
pub mod plain;
pub mod rename;
pub mod search;
pub mod snapshot;
pub mod tags;
pub mod ttl;
pub mod validate;
pub mod watch;

#[cfg(test)]
mod audit_tests;
#[cfg(test)]
mod history_tests;
#[cfg(test)]
mod merge_tests;
#[cfg(test)]
mod search_tests;
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tags_tests;
#[cfg(test)]
mod ttl_tests;
#[cfg(test)]
mod validate_tests;
#[cfg(test)]
mod watch_tests;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub type EnvMap = HashMap<String, String>;

pub fn read_env_file(path: &Path) -> Result<EnvMap> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read env file: {}", path.display()))?;
    parse_env(&content)
}

pub fn parse_env(content: &str) -> Result<EnvMap> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    Ok(map)
}

pub fn write_env_file(path: &Path, map: &EnvMap) -> Result<()> {
    let mut lines: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    lines.sort();
    let content = lines.join("\n") + "\n";
    fs::write(path, content)
        .with_context(|| format!("Failed to write env file: {}", path.display()))
}
