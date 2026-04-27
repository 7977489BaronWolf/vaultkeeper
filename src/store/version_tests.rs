#[cfg(test)]
mod tests {
    use super::super::version::VersionStore;

    fn ts() -> u64 {
        1_700_000_000
    }

    #[test]
    fn test_push_and_latest() {
        let mut store = VersionStore::new();
        store.push("API_KEY", "secret1", ts());
        let entry = store.latest("API_KEY").unwrap();
        assert_eq!(entry.value, "secret1");
        assert_eq!(entry.version, 1);
    }

    #[test]
    fn test_multiple_versions() {
        let mut store = VersionStore::new();
        store.push("DB_PASS", "pass1", ts());
        store.push("DB_PASS", "pass2", ts() + 10);
        store.push("DB_PASS", "pass3", ts() + 20);
        assert_eq!(store.count("DB_PASS"), 3);
        let latest = store.latest("DB_PASS").unwrap();
        assert_eq!(latest.version, 3);
        assert_eq!(latest.value, "pass3");
    }

    #[test]
    fn test_get_specific_version() {
        let mut store = VersionStore::new();
        store.push("TOKEN", "v1", ts());
        store.push("TOKEN", "v2", ts() + 5);
        let entry = store.get_version("TOKEN", 1).unwrap();
        assert_eq!(entry.value, "v1");
    }

    #[test]
    fn test_list_versions() {
        let mut store = VersionStore::new();
        store.push("KEY", "a", ts());
        store.push("KEY", "b", ts() + 1);
        let versions = store.list_versions("KEY");
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, 1);
        assert_eq!(versions[1].version, 2);
    }

    #[test]
    fn test_rollback() {
        let mut store = VersionStore::new();
        store.push("SECRET", "original", ts());
        store.push("SECRET", "updated", ts() + 10);
        let rolled = store.rollback("SECRET", 1);
        assert!(rolled);
        let latest = store.latest("SECRET").unwrap();
        assert_eq!(latest.value, "original");
        assert_eq!(latest.version, 3);
    }

    #[test]
    fn test_rollback_nonexistent_version() {
        let mut store = VersionStore::new();
        store.push("KEY", "val", ts());
        let rolled = store.rollback("KEY", 99);
        assert!(!rolled);
    }

    #[test]
    fn test_missing_key_returns_none() {
        let store = VersionStore::new();
        assert!(store.latest("MISSING").is_none());
        assert_eq!(store.list_versions("MISSING").len(), 0);
        assert_eq!(store.count("MISSING"), 0);
    }
}
