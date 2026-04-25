#[cfg(test)]
mod tests {
    use crate::store::relay::RelayStore;
    use crate::cli::relay::{
        handle_relay_add, handle_relay_remove, handle_relay_list, handle_relay_resolve,
    };
    use std::collections::HashMap;

    #[test]
    fn test_handle_relay_add_valid() {
        let mut store = RelayStore::new();
        handle_relay_add(&mut store, "DB_URL", vec!["APP_DB_URL".into()]);
        assert!(store.get_targets("DB_URL").is_some());
    }

    #[test]
    fn test_handle_relay_add_empty_source() {
        let mut store = RelayStore::new();
        // Should print error, not panic
        handle_relay_add(&mut store, "", vec!["TARGET".into()]);
        assert!(store.list().is_empty());
    }

    #[test]
    fn test_handle_relay_remove_existing() {
        let mut store = RelayStore::new();
        store.add_rule("KEY", vec!["VAL".into()]).unwrap();
        handle_relay_remove(&mut store, "KEY");
        assert!(store.get_targets("KEY").is_none());
    }

    #[test]
    fn test_handle_relay_remove_missing() {
        let mut store = RelayStore::new();
        // Should print error, not panic
        handle_relay_remove(&mut store, "GHOST");
    }

    #[test]
    fn test_handle_relay_list_empty() {
        let store = RelayStore::new();
        // Should print "No relay rules defined." without panicking
        handle_relay_list(&store);
    }

    #[test]
    fn test_handle_relay_list_with_rules() {
        let mut store = RelayStore::new();
        store.add_rule("A", vec!["B".into(), "C".into()]).unwrap();
        handle_relay_list(&store);
    }

    #[test]
    fn test_handle_relay_resolve_with_match() {
        let mut store = RelayStore::new();
        store.add_rule("SECRET", vec!["COPY_SECRET".into()]).unwrap();
        let mut secrets = HashMap::new();
        secrets.insert("SECRET".to_string(), "abc123".to_string());
        handle_relay_resolve(&store, &secrets);
    }

    #[test]
    fn test_handle_relay_resolve_no_match() {
        let store = RelayStore::new();
        let secrets = HashMap::new();
        handle_relay_resolve(&store, &secrets);
    }
}
