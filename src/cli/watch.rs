use crate::config::Config;
use crate::store::watch::WatchIndex;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn run_watch_status(config: &Config) -> Result<()> {
    let index = load_watch_index(config)?;
    if index.entries.is_empty() {
        println!("No environments are being watched.");
        return Ok(());
    }
    println!("{:<20} {:<10} {}", "NAME", "STATUS", "PATH");
    println!("{}", "-".repeat(60));
    for (name, entry) in &index.entries {
        let modified = index
            .is_modified(name, &entry.path)
            .unwrap_or(true);
        let status = if modified { "MODIFIED" } else { "OK" };
        println!("{:<20} {:<10} {}", name, status, entry.path.display());
    }
    Ok(())
}

pub fn run_watch_add(config: &Config, name: &str) -> Result<()> {
    let mut index = load_watch_index(config)?;
    let env_path = config.vault_dir.join(format!("{}.age", name));
    anyhow::ensure!(env_path.exists(), "Encrypted env '{}' not found", name);
    index
        .track(name, &env_path)
        .with_context(|| format!("Failed to track '{}'" , name))?;
    save_watch_index(config, &index)?;
    println!("Now watching: {}", name);
    Ok(())
}

pub fn run_watch_remove(config: &Config, name: &str) -> Result<()> {
    let mut index = load_watch_index(config)?;
    index.untrack(name);
    save_watch_index(config, &index)?;
    println!("Stopped watching: {}", name);
    Ok(())
}

fn watch_index_path(config: &Config) -> PathBuf {
    config.vault_dir.join(".watchindex.json")
}

fn load_watch_index(config: &Config) -> Result<WatchIndex> {
    let path = watch_index_path(config);
    if !path.exists() {
        return Ok(WatchIndex::new());
    }
    let data = std::fs::read_to_string(&path)
        .with_context(|| "Failed to read watch index")?;
    serde_json::from_str(&data).with_context(|| "Failed to parse watch index")
}

fn save_watch_index(config: &Config, index: &WatchIndex) -> Result<()> {
    let path = watch_index_path(config);
    let data = serde_json::to_string_pretty(index)?;
    std::fs::write(&path, data).with_context(|| "Failed to save watch index")
}
