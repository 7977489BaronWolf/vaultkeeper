use crate::config::Config;
use crate::store::history::History;
use anyhow::Result;
use std::path::PathBuf;

pub fn history_path(config: &Config) -> PathBuf {
    config.vault_dir.join(".history.json")
}

pub fn record(config: &Config, action: &str, key: &str, env: &str) -> Result<()> {
    let path = history_path(config);
    let mut h = History::load(&path)?;
    h.push(action, key, env);
    h.save(&path)?;
    Ok(())
}

pub fn handle_history(config: &Config, env: Option<&str>, limit: usize) -> Result<()> {
    let path = history_path(config);
    let h = History::load(&path)?;

    let entries: Vec<_> = match env {
        Some(e) => h.for_env(e).collect(),
        None => h.entries.iter().collect(),
    };

    if entries.is_empty() {
        println!("No history found.");
        return Ok(());
    }

    let display: Vec<_> = entries.iter().rev().take(limit).collect();
    println!("{:<30} {:<10} {:<10} {}", "TIMESTAMP", "ACTION", "ENV", "KEY");
    println!("{}", "-".repeat(65));
    for e in display.iter().rev() {
        println!(
            "{:<30} {:<10} {:<10} {}",
            e.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            e.action,
            e.env,
            e.key
        );
    }
    Ok(())
}

pub fn handle_history_clear(config: &Config) -> Result<()> {
    let path = history_path(config);
    let mut h = History::load(&path)?;
    h.clear();
    h.save(&path)?;
    println!("History cleared.");
    Ok(())
}
