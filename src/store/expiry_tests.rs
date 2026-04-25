#[cfg(test)]
mod tests {
    use super::super::expiry::*;
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn test_is_expired_past_timestamp() {
        let mut map = HashMap::new();
        map.insert("KEY".to_string(), now() - 10);
        assert!(is_expired("KEY", &map, false));
    }

    #[test]
    fn test_is_not_expired_future_timestamp() {
        let mut map = HashMap::new();
        map.insert("KEY".to_string(), now() + 9999);
        assert!(!is_expired("KEY", &map, false));
    }

    #[test]
    fn test_missing_key_respects_flag() {
        let map: HashMap<String, u64> = HashMap::new();
        assert!(is_expired("MISSING", &map, true));
        assert!(!is_expired("MISSING", &map, false));
    }

    #[test]
    fn test_expired_keys_returns_only_expired() {
        let mut map = HashMap::new();
        map.insert("OLD".to_string(), now() - 100);
        map.insert("NEW".to_string(), now() + 100);
        let expired = expired_keys(&map);
        assert_eq!(expired, vec!["OLD".to_string()]);
    }

    #[test]
    fn test_expiring_soon_within_window() {
        let mut map = HashMap::new();
        map.insert("SOON".to_string(), now() + 30);
        map.insert("LATER".to_string(), now() + 3600);
        let result = expiring_soon(&map, 60);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "SOON");
    }

    #[test]
    fn test_set_and_clear_expiry() {
        let mut map = HashMap::new();
        set_expiry("TOKEN", 300, &mut map);
        assert!(map.contains_key("TOKEN"));
        assert!(!is_expired("TOKEN", &map, false));
        let removed = clear_expiry("TOKEN", &mut map);
        assert!(removed);
        assert!(!map.contains_key("TOKEN"));
    }

    #[test]
    fn test_purge_expired_removes_stale_entries() {
        let mut map = HashMap::new();
        map.insert("STALE".to_string(), now() - 1);
        map.insert("FRESH".to_string(), now() + 1000);
        let purged = purge_expired(&mut map);
        assert_eq!(purged, vec!["STALE".to_string()]);
        assert!(!map.contains_key("STALE"));
        assert!(map.contains_key("FRESH"));
    }

    #[test]
    fn test_clear_expiry_nonexistent_returns_false() {
        let mut map: HashMap<String, u64> = HashMap::new();
        assert!(!clear_expiry("GHOST", &mut map));
    }
}
