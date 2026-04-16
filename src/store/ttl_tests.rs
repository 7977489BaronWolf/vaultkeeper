#[cfg(test)]
mod tests {
    use super::super::ttl::TtlStore;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_set_and_check_not_expired() {
        let mut store = TtlStore::new();
        store.set_ttl("API_KEY", 60);
        assert!(!store.is_expired("API_KEY"));
    }

    #[test]
    fn test_missing_key_not_expired() {
        let store = TtlStore::new();
        assert!(!store.is_expired("MISSING"));
    }

    #[test]
    fn test_expired_key() {
        let mut store = TtlStore::new();
        store.set_ttl("OLD_KEY", 0);
        thread::sleep(Duration::from_millis(10));
        assert!(store.is_expired("OLD_KEY"));
    }

    #[test]
    fn test_remove_ttl() {
        let mut store = TtlStore::new();
        store.set_ttl("KEY", 60);
        store.remove("KEY");
        assert!(!store.is_expired("KEY"));
    }

    #[test]
    fn test_expired_keys_list() {
        let mut store = TtlStore::new();
        store.set_ttl("LIVE", 60);
        store.set_ttl("DEAD", 0);
        thread::sleep(Duration::from_millis(10));
        let expired = store.expired_keys();
        assert!(expired.contains(&"DEAD".to_string()));
        assert!(!expired.contains(&"LIVE".to_string()));
    }

    #[test]
    fn test_ttl_remaining_some() {
        let mut store = TtlStore::new();
        store.set_ttl("KEY", 60);
        let remaining = store.ttl_remaining("KEY");
        assert!(remaining.is_some());
        assert!(remaining.unwrap() <= 60);
    }

    #[test]
    fn test_ttl_remaining_expired() {
        let mut store = TtlStore::new();
        store.set_ttl("KEY", 0);
        thread::sleep(Duration::from_millis(10));
        assert!(store.ttl_remaining("KEY").is_none());
    }
}
