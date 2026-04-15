#[cfg(test)]
mod tests {
    use crate::cli::set::run_set;
    use crate::store::VaultStore;
    use tempfile::tempdir;

    fn make_store(dir: &std::path::Path) -> VaultStore {
        VaultStore::new(&dir.join(".vault").to_string_lossy().to_string())
    }

    #[test]
    fn test_set_new_key() {
        let dir = tempdir().unwrap();
        let mut store = make_store(dir.path());

        let result = run_set(&mut store, "DB_URL", "postgres://localhost/mydb");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Set"));
        assert_eq!(store.get("DB_URL"), Some("postgres://localhost/mydb"));
    }

    #[test]
    fn test_set_updates_existing_key() {
        let dir = tempdir().unwrap();
        let mut store = make_store(dir.path());
        store.set("TOKEN", "old_value");

        let result = run_set(&mut store, "TOKEN", "new_value");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Updated"));
        assert_eq!(store.get("TOKEN"), Some("new_value"));
    }

    #[test]
    fn test_set_empty_key_fails() {
        let dir = tempdir().unwrap();
        let mut store = make_store(dir.path());

        let result = run_set(&mut store, "", "somevalue");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_set_key_with_equals_fails() {
        let dir = tempdir().unwrap();
        let mut store = make_store(dir.path());

        let result = run_set(&mut store, "BAD=KEY", "value");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("'='"));
    }

    #[test]
    fn test_set_multiple_keys() {
        let dir = tempdir().unwrap();
        let mut store = make_store(dir.path());

        run_set(&mut store, "KEY1", "val1").unwrap();
        run_set(&mut store, "KEY2", "val2").unwrap();
        run_set(&mut store, "KEY3", "val3").unwrap();

        assert_eq!(store.get("KEY1"), Some("val1"));
        assert_eq!(store.get("KEY2"), Some("val2"));
        assert_eq!(store.get("KEY3"), Some("val3"));
    }
}
