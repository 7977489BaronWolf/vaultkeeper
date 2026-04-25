use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns current Unix timestamp in seconds.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Check whether a secret key has expired given an expiry map.
/// Returns `true` if the key is expired or has no entry (treat missing as expired
/// only when `missing_is_expired` is true).
pub fn is_expired(key: &str, expiry_map: &HashMap<String, u64>, missing_is_expired: bool) -> bool {
    match expiry_map.get(key) {
        Some(&ts) => now_secs() >= ts,
        None => missing_is_expired,
    }
}

/// Return all keys from `expiry_map` that have already expired.
pub fn expired_keys(expiry_map: &HashMap<String, u64>) -> Vec<String> {
    let now = now_secs();
    expiry_map
        .iter()
        .filter(|(_, &ts)| now >= ts)
        .map(|(k, _)| k.clone())
        .collect()
}

/// Return all keys from `expiry_map` that will expire within `within_secs` seconds.
pub fn expiring_soon(expiry_map: &HashMap<String, u64>, within_secs: u64) -> Vec<(String, u64)> {
    let now = now_secs();
    let mut result: Vec<(String, u64)> = expiry_map
        .iter()
        .filter(|(_, &ts)| ts > now && ts - now <= within_secs)
        .map(|(k, &ts)| (k.clone(), ts - now))
        .collect();
    result.sort_by_key(|(_, remaining)| *remaining);
    result
}

/// Set an expiry timestamp for a key as `now + ttl_secs`.
pub fn set_expiry(key: &str, ttl_secs: u64, expiry_map: &mut HashMap<String, u64>) {
    expiry_map.insert(key.to_string(), now_secs() + ttl_secs);
}

/// Remove the expiry entry for a key (makes it non-expiring).
pub fn clear_expiry(key: &str, expiry_map: &mut HashMap<String, u64>) -> bool {
    expiry_map.remove(key).is_some()
}

/// Purge all expired entries from the map, returning the list of removed keys.
pub fn purge_expired(expiry_map: &mut HashMap<String, u64>) -> Vec<String> {
    let expired = expired_keys(expiry_map);
    for k in &expired {
        expiry_map.remove(k);
    }
    expired
}
