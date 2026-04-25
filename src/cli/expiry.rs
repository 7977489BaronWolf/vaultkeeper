use crate::store::expiry::{
    clear_expiry, expiring_soon, expired_keys, purge_expired, set_expiry,
};
use std::collections::HashMap;

/// Handle `vaultkeeper expiry set <key> <ttl_secs>`
pub fn handle_set(key: &str, ttl_secs: u64, expiry_map: &mut HashMap<String, u64>) {
    set_expiry(key, ttl_secs, expiry_map);
    println!("Expiry set for '{}': expires in {} seconds.", key, ttl_secs);
}

/// Handle `vaultkeeper expiry clear <key>`
pub fn handle_clear(key: &str, expiry_map: &mut HashMap<String, u64>) {
    if clear_expiry(key, expiry_map) {
        println!("Expiry cleared for '{}'.", key);
    } else {
        eprintln!("No expiry entry found for '{}'.", key);
    }
}

/// Handle `vaultkeeper expiry list`
pub fn handle_list(expiry_map: &HashMap<String, u64>) {
    if expiry_map.is_empty() {
        println!("No expiry entries configured.");
        return;
    }
    let mut entries: Vec<(&String, &u64)> = expiry_map.iter().collect();
    entries.sort_by_key(|(_, &ts)| ts);
    println!("{:<30} {}", "KEY", "EXPIRES AT (unix)");
    println!("{}", "-".repeat(50));
    for (key, ts) in entries {
        println!("{:<30} {}", key, ts);
    }
}

/// Handle `vaultkeeper expiry check <key>`
pub fn handle_check(key: &str, expiry_map: &HashMap<String, u64>) {
    use crate::store::expiry::is_expired;
    if is_expired(key, expiry_map, false) {
        println!("'{}' has EXPIRED or has no expiry set.", key);
    } else {
        println!("'{}' is still valid.", key);
    }
}

/// Handle `vaultkeeper expiry soon [--within <secs>]`
pub fn handle_soon(within_secs: u64, expiry_map: &HashMap<String, u64>) {
    let results = expiring_soon(expiry_map, within_secs);
    if results.is_empty() {
        println!("No secrets expiring within {} seconds.", within_secs);
        return;
    }
    println!("Secrets expiring within {} seconds:", within_secs);
    for (key, remaining) in results {
        println!("  {} — {} seconds remaining", key, remaining);
    }
}

/// Handle `vaultkeeper expiry purge`
pub fn handle_purge(expiry_map: &mut HashMap<String, u64>) {
    let purged = purge_expired(expiry_map);
    if purged.is_empty() {
        println!("Nothing to purge; no expired entries found.");
    } else {
        println!("Purged {} expired entr{}:", purged.len(), if purged.len() == 1 { "y" } else { "ies" });
        for key in &purged {
            println!("  - {}", key);
        }
    }
}

/// Handle `vaultkeeper expiry expired`
pub fn handle_expired(expiry_map: &HashMap<String, u64>) {
    let keys = expired_keys(expiry_map);
    if keys.is_empty() {
        println!("No expired secrets.");
    } else {
        println!("Expired secrets:");
        for key in &keys {
            println!("  - {}", key);
        }
    }
}
