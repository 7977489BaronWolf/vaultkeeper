#[cfg(test)]
mod tests {
    use super::super::pin_group::{PinGroup, PinGroupRegistry};
    use std::collections::HashSet;

    fn make_registry() -> PinGroupRegistry {
        let mut reg = PinGroupRegistry::new();
        reg.add_group(PinGroup::new("infra", vec!["DB_URL".into(), "REDIS_URL".into()]));
        reg.add_group(PinGroup::new("auth", vec!["JWT_SECRET".into(), "API_KEY".into()]));
        reg
    }

    #[test]
    fn test_add_and_get_group() {
        let reg = make_registry();
        let g = reg.get_group("infra").expect("infra group missing");
        assert!(g.contains("DB_URL"));
        assert!(g.contains("REDIS_URL"));
        assert!(!g.contains("JWT_SECRET"));
    }

    #[test]
    fn test_remove_group() {
        let mut reg = make_registry();
        let removed = reg.remove_group("auth");
        assert!(removed.is_some());
        assert!(reg.get_group("auth").is_none());
    }

    #[test]
    fn test_list_groups_sorted() {
        let reg = make_registry();
        let names: Vec<&str> = reg.list_groups().iter().map(|g| g.name.as_str()).collect();
        assert_eq!(names, vec!["auth", "infra"]);
    }

    #[test]
    fn test_groups_for_key() {
        let reg = make_registry();
        let groups = reg.groups_for_key("JWT_SECRET");
        assert_eq!(groups, vec!["auth"]);

        let none = reg.groups_for_key("NONEXISTENT");
        assert!(none.is_empty());
    }

    #[test]
    fn test_validate_all_present() {
        let reg = make_registry();
        let keys: HashSet<String> =
            vec!["DB_URL", "REDIS_URL", "JWT_SECRET", "API_KEY"]
                .into_iter()
                .map(String::from)
                .collect();
        let missing = reg.validate(&keys);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_validate_missing_keys() {
        let reg = make_registry();
        let keys: HashSet<String> =
            vec!["DB_URL"].into_iter().map(String::from).collect();
        let missing = reg.validate(&keys);
        assert_eq!(missing.len(), 3);
        assert!(missing.iter().any(|m| m.contains("REDIS_URL")));
        assert!(missing.iter().any(|m| m.contains("JWT_SECRET")));
        assert!(missing.iter().any(|m| m.contains("API_KEY")));
    }

    #[test]
    fn test_add_and_remove_key_from_group() {
        let mut group = PinGroup::new("ops", vec!["LOG_LEVEL".into()]);
        group.add_key("SENTRY_DSN");
        assert!(group.contains("SENTRY_DSN"));
        let removed = group.remove_key("LOG_LEVEL");
        assert!(removed);
        assert!(!group.contains("LOG_LEVEL"));
    }
}
