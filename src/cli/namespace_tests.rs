#[cfg(test)]
mod tests {
    use crate::cli::namespace::{
        cmd_namespace_add, cmd_namespace_assign, cmd_namespace_keys, cmd_namespace_list,
        cmd_namespace_remove,
    };
    use crate::store::namespace::NamespaceStore;

    fn base_store() -> NamespaceStore {
        let mut store = NamespaceStore::new();
        cmd_namespace_add(&mut store, "production", Some("Prod env")).unwrap();
        cmd_namespace_add(&mut store, "staging", None).unwrap();
        store
    }

    #[test]
    fn test_add_namespace_success() {
        let mut store = NamespaceStore::new();
        let result = cmd_namespace_add(&mut store, "dev", Some("Development"));
        assert!(result.is_ok());
        assert!(store.namespaces.contains_key("dev"));
    }

    #[test]
    fn test_add_namespace_duplicate_fails() {
        let mut store = base_store();
        let result = cmd_namespace_add(&mut store, "production", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_namespace_invalid_name_fails() {
        let mut store = NamespaceStore::new();
        let result = cmd_namespace_add(&mut store, "my namespace", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_namespace_success() {
        let mut store = base_store();
        let result = cmd_namespace_remove(&mut store, "staging");
        assert!(result.is_ok());
        assert!(!store.namespaces.contains_key("staging"));
    }

    #[test]
    fn test_remove_nonexistent_namespace_fails() {
        let mut store = base_store();
        let result = cmd_namespace_remove(&mut store, "ghost");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_namespaces_does_not_panic() {
        let store = base_store();
        cmd_namespace_list(&store);
    }

    #[test]
    fn test_list_empty_namespaces() {
        let store = NamespaceStore::new();
        cmd_namespace_list(&store);
    }

    #[test]
    fn test_assign_key_success() {
        let mut store = base_store();
        let result = cmd_namespace_assign(&mut store, "DB_URL", "production");
        assert!(result.is_ok());
        let ns = store.get_namespace_for_key("DB_URL").unwrap();
        assert_eq!(ns.name, "production");
    }

    #[test]
    fn test_assign_key_to_nonexistent_namespace_fails() {
        let mut store = base_store();
        let result = cmd_namespace_assign(&mut store, "API_KEY", "missing");
        assert!(result.is_err());
    }

    #[test]
    fn test_keys_in_namespace_does_not_panic() {
        let mut store = base_store();
        cmd_namespace_assign(&mut store, "SECRET_1", "production").unwrap();
        cmd_namespace_assign(&mut store, "SECRET_2", "production").unwrap();
        cmd_namespace_keys(&store, "production");
    }

    #[test]
    fn test_keys_in_empty_namespace() {
        let store = base_store();
        cmd_namespace_keys(&store, "staging");
    }
}
