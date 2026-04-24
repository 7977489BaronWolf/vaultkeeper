#[cfg(test)]
mod tests {
    use crate::store::policy::{Policy, PolicyRule, PolicyStore};

    #[test]
    fn test_policy_deny_keys() {
        let mut policy = Policy::new("no-secrets");
        policy.add_rule(PolicyRule::DenyKeys(vec!["PASSWORD".to_string()]));
        assert!(policy.evaluate("PASSWORD", "abc", "default").is_err());
        assert!(policy.evaluate("API_KEY", "abc", "default").is_ok());
    }

    #[test]
    fn test_policy_require_prefix() {
        let mut policy = Policy::new("prefix-check");
        policy.add_rule(PolicyRule::RequirePrefix("APP_".to_string()));
        assert!(policy.evaluate("APP_NAME", "foo", "default").is_ok());
        assert!(policy.evaluate("NAME", "foo", "default").is_err());
    }

    #[test]
    fn test_policy_max_value_length() {
        let mut policy = Policy::new("length-limit");
        policy.add_rule(PolicyRule::MaxValueLength(10));
        assert!(policy.evaluate("KEY", "short", "default").is_ok());
        assert!(policy.evaluate("KEY", "this_is_way_too_long_value", "default").is_err());
    }

    #[test]
    fn test_policy_allowed_namespaces() {
        let mut policy = Policy::new("ns-check");
        policy.add_rule(PolicyRule::AllowedNamespaces(vec![
            "prod".to_string(),
            "staging".to_string(),
        ]));
        assert!(policy.evaluate("KEY", "val", "prod").is_ok());
        assert!(policy.evaluate("KEY", "val", "dev").is_err());
    }

    #[test]
    fn test_policy_store_add_remove() {
        let mut store = PolicyStore::default();
        let policy = Policy::new("test-policy");
        store.add_policy(policy);
        assert!(store.get_policy("test-policy").is_some());
        assert!(store.remove_policy("test-policy"));
        assert!(store.get_policy("test-policy").is_none());
    }

    #[test]
    fn test_policy_store_evaluate_all() {
        let mut store = PolicyStore::default();
        let mut p1 = Policy::new("deny-pw");
        p1.add_rule(PolicyRule::DenyKeys(vec!["PASSWORD".to_string()]));
        let mut p2 = Policy::new("prefix");
        p2.add_rule(PolicyRule::RequirePrefix("APP_".to_string()));
        store.add_policy(p1);
        store.add_policy(p2);
        let errors = store.evaluate_all("PASSWORD", "val", "default");
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_policy_store_list_sorted() {
        let mut store = PolicyStore::default();
        store.add_policy(Policy::new("zebra"));
        store.add_policy(Policy::new("alpha"));
        store.add_policy(Policy::new("middle"));
        let list = store.list();
        assert_eq!(list[0].name, "alpha");
        assert_eq!(list[1].name, "middle");
        assert_eq!(list[2].name, "zebra");
    }
}
