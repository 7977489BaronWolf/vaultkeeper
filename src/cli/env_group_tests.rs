#[cfg(test)]
mod tests {
    use crate::store::env_group::{filter_by_group, missing_keys, parse_groups, EnvGroup};
    use std::collections::HashMap;

    fn make_group(name: &str, keys: &[&str]) -> EnvGroup {
        EnvGroup::new(name, keys.iter().map(|s| s.to_string()).collect())
    }

    fn make_secrets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_roundtrip_parse_and_filter() {
        let input = "[app]\nAPP_NAME\nAPP_VERSION\n";
        let groups = parse_groups(input);
        let secrets = make_secrets(&[("APP_NAME", "vaultkeeper"), ("APP_VERSION", "1.0")]);
        let filtered = filter_by_group(&secrets, &groups[0]);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered["APP_NAME"], "vaultkeeper");
    }

    #[test]
    fn test_multiple_groups_independent() {
        let input = "[db]\nDB_HOST\n[cache]\nREDIS_URL\n";
        let groups = parse_groups(input);
        let secrets = make_secrets(&[("DB_HOST", "localhost"), ("REDIS_URL", "redis://localhost")]);

        let db_filtered = filter_by_group(&secrets, &groups[0]);
        let cache_filtered = filter_by_group(&secrets, &groups[1]);

        assert!(db_filtered.contains_key("DB_HOST"));
        assert!(!db_filtered.contains_key("REDIS_URL"));
        assert!(cache_filtered.contains_key("REDIS_URL"));
        assert!(!cache_filtered.contains_key("DB_HOST"));
    }

    #[test]
    fn test_missing_keys_partial() {
        let secrets = make_secrets(&[("DB_HOST", "localhost")]);
        let group = make_group("db", &["DB_HOST", "DB_USER", "DB_PASS"]);
        let missing = missing_keys(&secrets, &group);
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&"DB_USER".to_string()));
        assert!(missing.contains(&"DB_PASS".to_string()));
    }

    #[test]
    fn test_group_with_no_keys() {
        let group = make_group("empty", &[]);
        let secrets = make_secrets(&[("SOME_KEY", "val")]);
        let filtered = filter_by_group(&secrets, &group);
        assert!(filtered.is_empty());
        let missing = missing_keys(&secrets, &group);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_parse_trims_whitespace() {
        let input = "  [mygroup]  \n  KEY_A  \n  KEY_B  \n";
        let groups = parse_groups(input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].keys, vec!["KEY_A", "KEY_B"]);
    }
}
