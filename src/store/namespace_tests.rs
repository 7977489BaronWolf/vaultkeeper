#[cfg(test)]
mod tests {
    use crate::store::namespace::{Namespace, NamespaceStore};

    fn make_store() -> NamespaceStore {
        let mut store = NamespaceStore::new();
        store
            .add_namespace(Namespace::new("production", Some("Prod secrets")))
            .unwrap();
        store
            .add_namespace(Namespace::new("staging", Some("Staging secrets")))
            .unwrap();
        store
    }

    #[test]
    fn test_valid_namespace_names() {
        assert!(Namespace::is_valid_name("prod"));
        assert!(Namespace::is_valid_name("my-namespace"));
        assert!(Namespace::is_valid_name("ns_01"));
        assert!(!Namespace::is_valid_name(""));
        assert!(!Namespace::is_valid_name("bad name"));
        assert!(!Namespace::is_valid_name("bad.name"));
    }

    #[test]
    fn test_add_namespace() {
        let mut store = NamespaceStore::new();
        let result = store.add_namespace(Namespace::new("dev", None));
        assert!(result.is_ok());
        assert!(store.namespaces.contains_key("dev"));
    }

    #[test]
    fn test_add_duplicate_namespace_fails() {
        let mut store = make_store();
        let result = store.add_namespace(Namespace::new("production", None));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_add_invalid_namespace_name_fails() {
        let mut store = NamespaceStore::new();
        let result = store.add_namespace(Namespace::new("bad name!", None));
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_namespace_unassigns_keys() {
        let mut store = make_store();
        store.assign("DB_URL", "staging").unwrap();
        store.remove_namespace("staging").unwrap();
        assert!(!store.namespaces.contains_key("staging"));
        assert!(!store.assignments.contains_key("DB_URL"));
    }

    #[test]
    fn test_remove_nonexistent_namespace_fails() {
        let mut store = make_store();
        let result = store.remove_namespace("ghost");
        assert!(result.is_err());
    }

    #[test]
    fn test_assign_and_lookup() {
        let mut store = make_store();
        store.assign("API_KEY", "production").unwrap();
        let ns = store.get_namespace_for_key("API_KEY").unwrap();
        assert_eq!(ns.name, "production");
    }

    #[test]
    fn test_assign_nonexistent_namespace_fails() {
        let mut store = make_store();
        let result = store.assign("SECRET", "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_keys_in_namespace() {
        let mut store = make_store();
        store.assign("DB_HOST", "production").unwrap();
        store.assign("DB_PASS", "production").unwrap();
        store.assign("STAGING_KEY", "staging").unwrap();
        let mut keys = store.keys_in_namespace("production");
        keys.sort();
        assert_eq!(keys, vec!["DB_HOST", "DB_PASS"]);
    }

    #[test]
    fn test_list_namespaces_sorted() {
        let store = make_store();
        let list = store.list_namespaces();
        let names: Vec<&str> = list.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(names, vec!["production", "staging"]);
    }
}
