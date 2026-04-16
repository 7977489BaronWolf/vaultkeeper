#[cfg(test)]
mod tests {
    use super::super::search::*;
    use std::collections::HashMap;

    fn sample() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("DATABASE_URL".into(), "postgres://localhost/db".into());
        m.insert("DATABASE_PASS".into(), "s3cr3t".into());
        m.insert("API_KEY".into(), "abc123".into());
        m.insert("APP_ENV".into(), "production".into());
        m
    }

    #[test]
    fn test_search_by_key_match() {
        let secrets = sample();
        let results = search_by_key(&secrets, "DATABASE");
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"DATABASE_URL".to_string()));
        assert!(results.contains(&"DATABASE_PASS".to_string()));
    }

    #[test]
    fn test_search_by_key_case_insensitive() {
        let secrets = sample();
        let results = search_by_key(&secrets, "api");
        assert_eq!(results, vec!["API_KEY"]);
    }

    #[test]
    fn test_search_by_key_no_match() {
        let secrets = sample();
        let results = search_by_key(&secrets, "REDIS");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_by_value() {
        let secrets = sample();
        let results = search_by_value(&secrets, "postgres");
        assert_eq!(results, vec!["DATABASE_URL"]);
    }

    #[test]
    fn test_search_by_value_no_match() {
        let secrets = sample();
        let results = search_by_value(&secrets, "notfound");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_entries_by_key() {
        let secrets = sample();
        let entries = search_entries(&secrets, "APP", false);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], ("APP_ENV".to_string(), "production".to_string()));
    }

    #[test]
    fn test_search_entries_by_value() {
        let secrets = sample();
        let entries = search_entries(&secrets, "abc123", true);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "API_KEY");
    }
}
