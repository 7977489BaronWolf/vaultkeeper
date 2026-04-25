use crate::store::lock_state::LockStateStore;

pub fn run_lock_key(
    store: &mut LockStateStore,
    key: &str,
    reason: Option<&str>,
) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("Key name cannot be empty.".to_string());
    }
    if store.is_locked(key) {
        return Err(format!("Key '{}' is already locked.", key));
    }
    store.lock_key(key, reason.map(|r| r.to_string()));
    println!("🔒 Key '{}' has been locked.", key);
    if let Some(r) = reason {
        println!("   Reason: {}", r);
    }
    Ok(())
}

pub fn run_unlock_key(
    store: &mut LockStateStore,
    key: &str,
) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err("Key name cannot be empty.".to_string());
    }
    if !store.is_locked(key) {
        return Err(format!("Key '{}' is not currently locked.", key));
    }
    store.unlock_key(key);
    println!("🔓 Key '{}' has been unlocked.", key);
    Ok(())
}

pub fn run_list_locked(store: &LockStateStore) {
    let mut locked = store.all_locked_keys();
    locked.sort();
    if locked.is_empty() {
        println!("No keys are currently locked.");
        return;
    }
    println!("Locked keys ({}):", locked.len());
    for key in locked {
        let entry = store.get_entry(key).unwrap();
        match &entry.reason {
            Some(r) => println!("  🔒 {} — {}", key, r),
            None => println!("  🔒 {}", key),
        }
    }
}
