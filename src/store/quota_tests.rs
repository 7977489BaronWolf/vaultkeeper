#[cfg(test)]
mod tests {
    use super::super::quota::*;
    use std::collections::HashMap;

    fn make_secrets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_quota_ok_within_limits() {
        let secrets = make_secrets(&[("FOO", "bar"), ("BAZ", "qux")]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert!(report.is_ok());
        assert_eq!(report.key_count, 2);
    }

    #[test]
    fn test_quota_exceeds_key_count() {
        let pairs: Vec<(String, String)> = (0..5).map(|i| (format!("KEY_{}", i), "v".to_string())).collect();
        let secrets: HashMap<String, String> = pairs.into_iter().collect();
        let config = QuotaConfig { max_keys: 3, ..QuotaConfig::default() };
        let report = check_quota(&secrets, &config);
        assert!(!report.is_ok());
        assert!(report.violations.iter().any(|v| v.contains("key count")));
    }

    #[test]
    fn test_quota_value_too_large() {
        let big_val = "x".repeat(100);
        let secrets = make_secrets(&[("BIG", &big_val)]);
        let config = QuotaConfig { max_value_bytes: 50, ..QuotaConfig::default() };
        let report = check_quota(&secrets, &config);
        assert!(!report.is_ok());
        assert!(report.violations.iter().any(|v| v.contains("BIG")));
    }

    #[test]
    fn test_quota_total_bytes_exceeded() {
        let secrets = make_secrets(&[("A", &"z".repeat(200)), ("B", &"z".repeat(200))]);
        let config = QuotaConfig { max_total_bytes: 100, ..QuotaConfig::default() };
        let report = check_quota(&secrets, &config);
        assert!(!report.is_ok());
        assert!(report.violations.iter().any(|v| v.contains("total size")));
    }

    #[test]
    fn test_quota_largest_key_tracked() {
        let secrets = make_secrets(&[("SMALL", "ab"), ("LARGE", "abcdefghij")]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert_eq!(report.largest_key.as_deref(), Some("LARGE"));
        assert_eq!(report.largest_value_bytes, 10);
    }

    #[test]
    fn test_quota_empty_secrets() {
        let secrets = HashMap::new();
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert!(report.is_ok());
        assert_eq!(report.key_count, 0);
        assert_eq!(report.total_bytes, 0);
        assert!(report.largest_key.is_none());
    }
}
