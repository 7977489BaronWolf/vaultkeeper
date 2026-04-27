#[cfg(test)]
mod tests {
    use crate::store::lock_state::{LockStateStore, LockStatus};

    #[test]
    fn test_lock_key() {
        let mut store = LockStateStore::new();
        store.lock_key("DB_PASSWORD", Some("sensitive".to_string()));
        assert!(store.is_locked("DB_PASSWORD"));
    }

    #[test]
    fn test_unlock_key() {
        let mut store = LockStateStore::new();
        store.lock_key("API_KEY", None);
        assert!(store.is_locked("API_KEY"));
        store.unlock_key("API_KEY");
        assert!(!store.is_locked("API_KEY"));
    }

    #[test]
    fn test_unlocked_by_default() {
        let store = LockStateStore::new();
        assert!(!store.is_locked("UNKNOWN_KEY"));
    }

    #[test]
    fn test_all_locked_keys() {
        let mut store = LockStateStore::new();
        store.lock_key("SECRET_A", None);
        store.lock_key("SECRET_B", Some("reason".to_string()));
        store.unlock_key("SECRET_A");
        let locked = store.all_locked_keys();
        assert_eq!(locked.len(), 1);
        assert!(locked.contains(&"SECRET_B"));
    }

    #[test]
    fn test_get_entry_reason() {
        let mut store = LockStateStore::new();
        store.lock_key("TOKEN", Some("do not modify".to_string()));
        let entry = store.get_entry("TOKEN").unwrap();
        assert_eq!(entry.reason.as_deref(), Some("do not modify"));
        assert!(entry.locked_at.is_some());
    }

    #[test]
    fn test_remove_entry() {
        let mut store = LockStateStore::new();
        store.lock_key("TEMP_KEY", None);
        store.remove("TEMP_KEY");
        assert!(!store.is_locked("TEMP_KEY"));
        assert!(store.get_entry("TEMP_KEY").is_none());
    }

    #[test]
    fn test_lock_entry_status() {
        let mut store = LockStateStore::new();
        store.lock_key("X", None);
        let entry = store.get_entry("X").unwrap();
        assert_eq!(entry.status, LockStatus::Locked);
        store.unlock_key("X");
        let entry = store.get_entry("X").unwrap();
        assert_eq!(entry.status, LockStatus::Unlocked);
    }

    #[test]
    fn test_lock_key_no_reason() {
        // Locking a key without a reason should store None for the reason field
        let mut store = LockStateStore::new();
        store.lock_key("NO_REASON_KEY", None);
        let entry = store.get_entry("NO_REASON_KEY").unwrap();
        assert!(entry.reason.is_none());
        assert_eq!(entry.status, LockStatus::Locked);
    }

    #[test]
    fn test_relock_key_updates_reason() {
        // Re-locking an already locked key should update the reason
        let mut store = LockStateStore::new();
        store.lock_key("RELOCK_KEY", Some("initial reason".to_string()));
        store.lock_key("RELOCK_KEY", Some("updated reason".to_string()));
        let entry = store.get_entry("RELOCK_KEY").unwrap();
        assert_eq!(entry.reason.as_deref(), Some("updated reason"));
        assert_eq!(entry.status, LockStatus::Locked);
    }
}
