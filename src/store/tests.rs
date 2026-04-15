#[cfg(test)]
mod tests {
    use super::super::*;
    use tempfile::tempdir;

    fn make_entry(name: &str) -> VaultEntry {
        VaultEntry {
            name: name.to_string(),
            encrypted_file: format!("{}.age", name),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_load_or_create_new() {
        let dir = tempdir().unwrap();
        let store = VaultStore::load_or_create(dir.path()).unwrap();
        assert!(store.entries.is_empty());
    }

    #[test]
    fn test_insert_and_list() {
        let dir = tempdir().unwrap();
        let mut store = VaultStore::load_or_create(dir.path()).unwrap();
        store.insert(make_entry("DB_PASSWORD"));
        store.insert(make_entry("API_KEY"));
        let entries = store.list_entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], "API_KEY");
        assert_eq!(entries[1], "DB_PASSWORD");
    }

    #[test]
    fn test_save_and_reload() {
        let dir = tempdir().unwrap();
        let mut store = VaultStore::load_or_create(dir.path()).unwrap();
        store.insert(make_entry("SECRET_TOKEN"));
        store.save().unwrap();

        let reloaded = VaultStore::load(dir.path()).unwrap();
        assert!(reloaded.get("SECRET_TOKEN").is_some());
        assert_eq!(reloaded.entries.len(), 1);
    }

    #[test]
    fn test_load_missing_vault_errors() {
        let dir = tempdir().unwrap();
        let result = VaultStore::load(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_existing_entry() {
        let dir = tempdir().unwrap();
        let mut store = VaultStore::load_or_create(dir.path()).unwrap();
        store.insert(make_entry("MY_KEY"));
        let entry = store.get("MY_KEY").unwrap();
        assert_eq!(entry.encrypted_file, "MY_KEY.age");
    }
}
