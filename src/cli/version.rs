use crate::store::version::VersionStore;

pub fn cmd_version_list(store: &VersionStore, key: &str) {
    let versions = store.list_versions(key);
    if versions.is_empty() {
        eprintln!("No versions found for key: {}", key);
        return;
    }
    println!("Versions for '{}':", key);
    for entry in versions {
        println!(
            "  v{} — {} (ts: {})",
            entry.version,
            mask_value(&entry.value),
            entry.created_at
        );
    }
}

pub fn cmd_version_get(store: &VersionStore, key: &str, version: u32) {
    match store.get_version(key, version) {
        Some(entry) => println!("{}", entry.value),
        None => eprintln!("Version {} not found for key '{}'.", version, key),
    }
}

pub fn cmd_version_rollback(store: &mut VersionStore, key: &str, version: u32) {
    if store.rollback(key, version) {
        println!("Rolled back '{}' to version {}.", key, version);
    } else {
        eprintln!("Failed to rollback '{}': version {} not found.", key, version);
    }
}

pub fn cmd_version_latest(store: &VersionStore, key: &str) {
    match store.latest(key) {
        Some(entry) => println!("v{}: {}", entry.version, entry.value),
        None => eprintln!("No entries found for key '{}'.", key),
    }
}

fn mask_value(value: &str) -> String {
    if value.len() <= 4 {
        return "****".to_string();
    }
    let visible = &value[..2];
    format!("{}****", visible)
}
