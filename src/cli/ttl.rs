use crate::config::Config;
use crate::store::ttl::TtlStore;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn ttl_path(config: &Config) -> PathBuf {
    config.vault_dir.join(".ttl.json")
}

pub fn load_ttl_store(config: &Config) -> Result<TtlStore> {
    let path = ttl_path(config);
    if !path.exists() {
        return Ok(TtlStore::new());
    }
    let data = fs::read_to_string(&path).context("Failed to read TTL store")?;
    serde_json::from_str(&data).context("Failed to parse TTL store")
}

pub fn save_ttl_store(config: &Config, store: &TtlStore) -> Result<()> {
    let path = ttl_path(config);
    let data = serde_json::to_string_pretty(store)?;
    fs::write(&path, data).context("Failed to write TTL store")
}

pub fn handle_ttl_set(config: &Config, key: &str, seconds: u64) -> Result<()> {
    let mut store = load_ttl_store(config)?;
    store.set_ttl(key, seconds);
    save_ttl_store(config, &store)?;
    println!("TTL set: '{}' expires in {}s", key, seconds);
    Ok(())
}

pub fn handle_ttl_check(config: &Config, key: &str) -> Result<()> {
    let store = load_ttl_store(config)?;
    if store.is_expired(key) {
        println!("'{}' has expired", key);
    } else if let Some(remaining) = store.ttl_remaining(key) {
        println!("'{}' expires in {}s", key, remaining);
    } else {
        println!("'{}' has no TTL set", key);
    }
    Ok(())
}

pub fn handle_ttl_purge(config: &Config) -> Result<()> {
    let mut store = load_ttl_store(config)?;
    let expired = store.expired_keys();
    let count = expired.len();
    for key in &expired {
        store.remove(key);
    }
    save_ttl_store(config, &store)?;
    println!("Purged {} expired TTL entries", count);
    Ok(())
}

pub fn handle_ttl_list(config: &Config) -> Result<()> {
    let store = load_ttl_store(config)?;
    if store.entries.is_empty() {
        println!("No TTL entries set.");
        return Ok(());
    }
    for (key, exp) in &store.entries {
        println!("{} => expires_at={}", key, exp);
    }
    Ok(())
}
