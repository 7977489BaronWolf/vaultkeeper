use crate::store::profile::{Profile, ProfileStore};
use std::path::Path;

fn load_store(path: &Path) -> ProfileStore {
    if path.exists() {
        let data = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        ProfileStore::new()
    }
}

fn save_store(path: &Path, store: &ProfileStore) -> Result<(), String> {
    let data = serde_json::to_string_pretty(store)
        .map_err(|e| format!("serialize error: {e}"))?;
    std::fs::write(path, data).map_err(|e| format!("write error: {e}"))
}

pub fn handle_profile_add(store_path: &Path, name: &str, description: Option<&str>) -> Result<(), String> {
    let mut store = load_store(store_path);
    let mut profile = Profile::new(name);
    if let Some(desc) = description {
        profile = profile.with_description(desc);
    }
    store.add(profile)?;
    save_store(store_path, &store)?;
    println!("Profile '{}' created.", name);
    Ok(())
}

pub fn handle_profile_remove(store_path: &Path, name: &str) -> Result<(), String> {
    let mut store = load_store(store_path);
    store.remove(name)?;
    save_store(store_path, &store)?;
    println!("Profile '{}' removed.", name);
    Ok(())
}

pub fn handle_profile_activate(store_path: &Path, name: &str) -> Result<(), String> {
    let mut store = load_store(store_path);
    store.activate(name)?;
    save_store(store_path, &store)?;
    println!("Profile '{}' is now active.", name);
    Ok(())
}

pub fn handle_profile_deactivate(store_path: &Path) -> Result<(), String> {
    let mut store = load_store(store_path);
    store.deactivate();
    save_store(store_path, &store)?;
    println!("No active profile.");
    Ok(())
}

pub fn handle_profile_list(store_path: &Path) -> Result<(), String> {
    let store = load_store(store_path);
    if store.profiles.is_empty() {
        println!("No profiles defined.");
        return Ok(());
    }
    for profile in store.list() {
        let active_marker = if store.active.as_deref() == Some(&profile.name) { " (active)" } else { "" };
        let desc = profile.description.as_deref().unwrap_or("");
        println!("  {}{} — {}", profile.name, active_marker, desc);
    }
    Ok(())
}

pub fn handle_profile_show(store_path: &Path, name: &str) -> Result<(), String> {
    let store = load_store(store_path);
    let profile = store.profiles.get(name)
        .ok_or_else(|| format!("profile '{}' not found", name))?;
    println!("Name: {}", profile.name);
    if let Some(desc) = &profile.description {
        println!("Description: {}", desc);
    }
    if profile.env_overrides.is_empty() {
        println!("Overrides: (none)");
    } else {
        println!("Overrides:");
        let mut keys: Vec<&String> = profile.env_overrides.keys().collect();
        keys.sort();
        for k in keys {
            println!("  {}={}", k, profile.env_overrides[k]);
        }
    }
    Ok(())
}
