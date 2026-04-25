#[cfg(test)]
mod tests {
    use super::super::relay::RelayStore;
    use std::collections::HashMap;

    fn make_secrets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_add_and_get_rule() {
        let mut store = RelayStore::new();
        store.add_rule("DB_URL", vec!["APP_DB_URL".into(), "WORKER_DB_URL".into()]).unwrap();
        let targets = store.get_targets("DB_URL").unwrap();
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&"APP_DB_URL".to_string()));
    }

    #[test]
    fn test_add_rule_replaces_existing() {
        let mut store = RelayStore::new();
        store.add_rule("KEY", vec!["A".into()]).unwrap();
        store.add_rule("KEY", vec!["B".into(), "C".into()]).unwrap();
        assert_eq!(store.list().len(), 1);
        assert_eq!(store.get_targets("KEY").unwrap().len(), 2);
    }

    #[test]
    fn test_add_rule_empty_source_fails() {
        let mut store = RelayStore::new();
        assert!(store.add_rule("", vec!["A".into()]).is_err());
    }

    #[test]
    fn test_add_rule_empty_targets_fails() {
        let mut store = RelayStore::new();
        assert!(store.add_rule("KEY", vec![]).is_err());
    }

    #[test]
    fn test_remove_rule() {
        let mut store = RelayStore::new();
        store.add_rule("X", vec!["Y".into()]).unwrap();
        assert!(store.remove_rule("X"));
        assert!(store.get_targets("X").is_none());
    }

    #[test]
    fn test_remove_nonexistent_rule() {
        let mut store = RelayStore::new();
        assert!(!store.remove_rule("GHOST"));
    }

    #[test]
    fn test_resolve_propagates_values() {
        let mut store = RelayStore::new();
        store.add_rule("DB_PASS", vec!["API_DB_PASS".into(), "JOB_DB_PASS".into()]).unwrap();
        let secrets = make_secrets(&[("DB_PASS", "supersecret")]);
        let resolved = store.resolve(&secrets);
        assert_eq!(resolved.get("API_DB_PASS").unwrap(), "supersecret");
        assert_eq!(resolved.get("JOB_DB_PASS").unwrap(), "supersecret");
    }

    #[test]
    fn test_resolve_skips_missing_source() {
        let mut store = RelayStore::new();
        store.add_rule("MISSING_KEY", vec!["TARGET".into()]).unwrap();
        let secrets = make_secrets(&[]);
        let resolved = store.resolve(&secrets);
        assert!(resolved.is_empty());
    }
}
