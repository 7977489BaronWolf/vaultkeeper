#[cfg(test)]
mod tests {
    use crate::cli::lock_key::{run_lock_key, run_unlock_key, run_list_locked};
    use crate::store::lock_state::LockStateStore;

    #[test]
    fn test_lock_key_success() {
        let mut store = LockStateStore::new();
        let result = run_lock_key(&mut store, "DB_PASS", Some("production secret"));
        assert!(result.is_ok());
        assert!(store.is_locked("DB_PASS"));
    }

    #[test]
    fn test_lock_key_already_locked() {
        let mut store = LockStateStore::new();
        run_lock_key(&mut store, "API_KEY", None).unwrap();
        let result = run_lock_key(&mut store, "API_KEY", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already locked"));
    }

    #[test]
    fn test_lock_key_empty_name() {
        let mut store = LockStateStore::new();
        let result = run_lock_key(&mut store, "", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_unlock_key_success() {
        let mut store = LockStateStore::new();
        run_lock_key(&mut store, "TOKEN", None).unwrap();
        let result = run_unlock_key(&mut store, "TOKEN");
        assert!(result.is_ok());
        assert!(!store.is_locked("TOKEN"));
    }

    #[test]
    fn test_unlock_key_not_locked() {
        let mut store = LockStateStore::new();
        let result = run_unlock_key(&mut store, "MISSING");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not currently locked"));
    }

    #[test]
    fn test_unlock_key_empty_name() {
        let mut store = LockStateStore::new();
        let result = run_unlock_key(&mut store, "   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_locked_empty() {
        let store = LockStateStore::new();
        // Should not panic
        run_list_locked(&store);
    }

    #[test]
    fn test_list_locked_with_entries() {
        let mut store = LockStateStore::new();
        run_lock_key(&mut store, "KEY_A", Some("reason a")).unwrap();
        run_lock_key(&mut store, "KEY_B", None).unwrap();
        // Should not panic and should show both keys
        run_list_locked(&store);
        assert_eq!(store.all_locked_keys().len(), 2);
    }
}
