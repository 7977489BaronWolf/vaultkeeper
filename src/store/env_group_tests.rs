#[cfg(test)]
mod tests {
    use super::super::env_group::*;
    use std::collections::HashMap;

    fn sample_secrets() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("DB_HOST".to_string(), "localhost".to_string());
        m.insert("DB_PORT".to_string(), "5432".to_string());
        m.insert("API_KEY".to_string(), "secret".to_string());
        m.insert("LOG_LEVEL".to_string(), "info".to_string());
        m
    }

    #[test]
    fn test_parse_groups_basic() {
        let input = "[database]\nDB_HOST\nDB_PORT\n\n[api]\nAPI_KEY\n";
        let groups = parse_groups(input);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].name, "database");
        assert_eq!(groups[0].keys, vec!["DB_HOST", "DB_PORT"]);
        assert_eq!(groups[1].name, "api");
        assert_eq!(groups[1].keys, vec!["API_KEY"]);
    }

    #[test]
    fn test_parse_groups_skips_comments() {
        let input = "# comment\n[logging]\n# skip\nLOG_LEVEL\n";
        let groups = parse_groups(input);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].keys, vec!["LOG_LEVEL"]);
    }

    #[test]
    fn test_parse_groups_empty() {
        let groups = parse_groups("");
        assert!(groups.is_empty());
    }

    #[test]
    fn test_filter_by_group() {
        let secrets = sample_secrets();
        let group = EnvGroup::new("database", vec!["DB_HOST".to_string(), "DB_PORT".to_string()]);
        let filtered = filter_by_group(&secrets, &group);
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains_key("DB_HOST"));
        assert!(filtered.contains_key("DB_PORT"));
        assert!(!filtered.contains_key("API_KEY"));
    }

    #[test]
    fn test_filter_by_group_no_match() {
        let secrets = sample_secrets();
        let group = EnvGroup::new("other", vec!["MISSING_KEY".to_string()]);
        let filtered = filter_by_group(&secrets, &group);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_missing_keys() {
        let secrets = sample_secrets();
        let group = EnvGroup::new(
            "database",
            vec!["DB_HOST".to_string(), "DB_PASS".to_string()],
        );
        let missing = missing_keys(&secrets, &group);
        assert_eq!(missing, vec!["DB_PASS"]);
    }

    #[test]
    fn test_missing_keys_none_missing() {
        let secrets = sample_secrets();
        let group = EnvGroup::new("database", vec!["DB_HOST".to_string()]);
        let missing = missing_keys(&secrets, &group);
        assert!(missing.is_empty());
    }
}
