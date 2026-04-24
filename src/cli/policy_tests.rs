#[cfg(test)]
mod tests {
    use crate::cli::policy::{
        run_policy_add, run_policy_check, run_policy_list, run_policy_remove,
    };
    use crate::store::policy::PolicyStore;

    #[test]
    fn test_add_deny_keys_rule() {
        let mut store = PolicyStore::default();
        let result = run_policy_add(&mut store, "my-policy", "deny-keys:SECRET,TOKEN");
        assert!(result.is_ok());
        let policy = store.get_policy("my-policy").unwrap();
        assert_eq!(policy.rules.len(), 1);
    }

    #[test]
    fn test_add_require_prefix_rule() {
        let mut store = PolicyStore::default();
        let result = run_policy_add(&mut store, "prefix-policy", "require-prefix:APP_");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_max_value_length_rule() {
        let mut store = PolicyStore::default();
        let result = run_policy_add(&mut store, "len-policy", "max-value-length:64");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_allowed_namespaces_rule() {
        let mut store = PolicyStore::default();
        let result =
            run_policy_add(&mut store, "ns-policy", "allowed-namespaces:prod,staging");
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_unknown_rule_fails() {
        let mut store = PolicyStore::default();
        let result = run_policy_add(&mut store, "p", "unknown-rule:foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_existing_policy() {
        let mut store = PolicyStore::default();
        run_policy_add(&mut store, "to-remove", "deny-keys:FOO").unwrap();
        let result = run_policy_remove(&mut store, "to-remove");
        assert!(result.is_ok());
        assert!(store.get_policy("to-remove").is_none());
    }

    #[test]
    fn test_remove_missing_policy_fails() {
        let mut store = PolicyStore::default();
        let result = run_policy_remove(&mut store, "ghost");
        assert!(result.is_err());
    }

    #[test]
    fn test_policy_check_passes() {
        let mut store = PolicyStore::default();
        run_policy_add(&mut store, "p", "require-prefix:APP_").unwrap();
        let result = run_policy_check(&store, "APP_NAME", "myapp", "default");
        assert!(result.is_ok());
    }

    #[test]
    fn test_policy_check_fails() {
        let mut store = PolicyStore::default();
        run_policy_add(&mut store, "p", "deny-keys:PASSWORD").unwrap();
        let result = run_policy_check(&store, "PASSWORD", "secret", "default");
        assert!(result.is_err());
    }

    #[test]
    fn test_policy_list_empty() {
        let store = PolicyStore::default();
        run_policy_list(&store); // should not panic
    }
}
