use crate::config::Config;
use crate::store::hook::{HookEvent, HookStore};
use anyhow::{Context, Result};
use std::path::PathBuf;

fn hook_store_path(config: &Config) -> PathBuf {
    config.vault_dir.join(".hooks.json")
}

fn load_hook_store(config: &Config) -> Result<HookStore> {
    let path = hook_store_path(config);
    if !path.exists() {
        return Ok(HookStore::default());
    }
    let data = std::fs::read_to_string(&path).context("Failed to read hooks file")?;
    serde_json::from_str(&data).context("Failed to parse hooks file")
}

fn save_hook_store(config: &Config, store: &HookStore) -> Result<()> {
    let path = hook_store_path(config);
    let data = serde_json::to_string_pretty(store).context("Failed to serialize hooks")?;
    std::fs::write(&path, data).context("Failed to write hooks file")
}

pub fn handle_hook_add(config: &Config, event: &str, command: &str) -> Result<()> {
    let hook_event = parse_event(event)?;
    let mut store = load_hook_store(config)?;
    store.add_hook(hook_event.clone(), command.to_string());
    save_hook_store(config, &store)?;
    println!("Hook added for event '{}': {}", hook_event, command);
    Ok(())
}

pub fn handle_hook_remove(config: &Config, event: &str, command: &str) -> Result<()> {
    let hook_event = parse_event(event)?;
    let mut store = load_hook_store(config)?;
    if store.remove_hook(&hook_event, command) {
        save_hook_store(config, &store)?;
        println!("Hook removed.");
    } else {
        println!("No matching hook found.");
    }
    Ok(())
}

pub fn handle_hook_list(config: &Config) -> Result<()> {
    let store = load_hook_store(config)?;
    let hooks = store.list_hooks();
    if hooks.is_empty() {
        println!("No hooks configured.");
    } else {
        println!("{:<15} {:<8} {}", "EVENT", "ENABLED", "COMMAND");
        for h in hooks {
            println!("{:<15} {:<8} {}", h.event, h.enabled, h.command);
        }
    }
    Ok(())
}

fn parse_event(s: &str) -> Result<HookEvent> {
    match s {
        "pre-lock" => Ok(HookEvent::PreLock),
        "post-lock" => Ok(HookEvent::PostLock),
        "pre-unlock" => Ok(HookEvent::PreUnlock),
        "post-unlock" => Ok(HookEvent::PostUnlock),
        "post-set" => Ok(HookEvent::PostSet),
        "post-delete" => Ok(HookEvent::PostDelete),
        "post-rotate" => Ok(HookEvent::PostRotate),
        _ => Err(anyhow::anyhow!("Unknown hook event: '{}'. Valid events: pre-lock, post-lock, pre-unlock, post-unlock, post-set, post-delete, post-rotate", s)),
    }
}
