#[cfg(test)]
mod tests {
    use crate::cli::delete::run_delete;
    use crate::store::VaultStore;
    use tempfile::tempdir;

    fn make_store_with_keys(dir: &std::path::Path) -> VaultStore {
        let mut store = VaultStore::new(&dir.join(".vault").to_string_lossy().to_string());
        store.set("API_KEY", "abc123");
        store.set("SECRET", "shh");
        store.set("TOKEN", "tok_xyz");
        store
    }

    #[test]
    fn test_delete_existing_key() {
        let dir = tempdir().unwrap();
        let mut store = make_store_with_keys(dir.path());

        let result = run_delete(&mut store, "API_KEY");
        assert!(result.is_ok());
        assert!(store.get("API_KEY").is_none());
    }

    #[test]
    fn test_delete_nonexistent_key_returns_error() {
        let dir = tempdir().unwrap();
        let mut store = make_store_with_keys(dir.path());

        let result = run_delete(&mut store, "MISSING_KEY");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MISSING_KEY"));
    }

    #[test]
    fn test_delete_does_not_affect_other_keys() {
        let dir = tempdir().unwrap();
        let mut store = make_store_with_keys(dir.path());

        run_delete(&mut store, "SECRET").unwrap();
        assert!(store.get("SECRET").is_none());
        assert_eq!(store.get("API_KEY"), Some("abc123"));
        assert_eq!(store.get("TOKEN"), Some("tok_xyz"));
    }

    #[test]
    fn test_delete_last_key_leaves_empty_store() {
        let dir = tempdir().unwrap();
        let mut store = VaultStore::new(&dir.path().join(".vault").to_string_lossy().to_string());
        store.set("ONLY_KEY", "value");

        run_delete(&mut store, "ONLY_KEY").unwrap();
        assert!(store.keys().is_empty());
    }

    #[test]
    fn test_delete_already_deleted_key_errors() {
        let dir = tempdir().unwrap();
        let mut store = make_store_with_keys(dir.path());

        run_delete(&mut store, "TOKEN").unwrap();
        let second = run_delete(&mut store, "TOKEN");
        assert!(second.is_err());
    }
}
