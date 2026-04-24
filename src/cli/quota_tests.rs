#[cfg(test)]
mod tests {
    use crate::store::quota::{check_quota, QuotaConfig, QuotaReport};
    use std::collections::HashMap;

    fn make_secrets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_report_is_ok_no_violations() {
        let secrets = make_secrets(&[("KEY", "value")]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert!(report.is_ok());
        assert!(report.violations.is_empty());
    }

    #[test]
    fn test_report_multiple_violations() {
        let big = "x".repeat(5000);
        let mut secrets = HashMap::new();
        for i in 0..10 {
            secrets.insert(format!("K{}", i), big.clone());
        }
        let config = QuotaConfig {
            max_keys: 5,
            max_value_bytes: 100,
            max_total_bytes: 200,
        };
        let report = check_quota(&secrets, &config);
        assert!(!report.is_ok());
        assert!(report.violations.len() >= 2);
    }

    #[test]
    fn test_quota_config_defaults() {
        let config = QuotaConfig::default();
        assert_eq!(config.max_keys, 500);
        assert_eq!(config.max_value_bytes, 4096);
        assert_eq!(config.max_total_bytes, 1_048_576);
    }

    #[test]
    fn test_report_fields_populated() {
        let secrets = make_secrets(&[("ALPHA", "hello"), ("BETA", "world!")]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert_eq!(report.key_count, 2);
        assert!(report.total_bytes > 0);
        assert!(report.largest_key.is_some());
        assert_eq!(report.largest_value_bytes, 6); // "world!"
    }

    #[test]
    fn test_exact_boundary_ok() {
        let val = "v".repeat(4096);
        let secrets = make_secrets(&[("K", &val)]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert!(report.is_ok(), "should be ok at exact boundary");
    }

    #[test]
    fn test_one_over_boundary_fails() {
        let val = "v".repeat(4097);
        let secrets = make_secrets(&[("K", &val)]);
        let config = QuotaConfig::default();
        let report = check_quota(&secrets, &config);
        assert!(!report.is_ok());
    }
}
