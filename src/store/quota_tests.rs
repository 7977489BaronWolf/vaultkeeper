#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::store::quota::{check_insert, usage_summary, Quota, QuotaError};

    fn make_secrets(n: usize) -> HashMap<String, String> {
        (0..n).map(|i| (format!("KEY_{i}"), "value".to_string())).collect()
    }

    #[test]
    fn test_insert_within_limits() {
        let quota = Quota::default();
        let secrets = make_secrets(5);
        assert!(check_insert(&quota, &secrets, "NEW_KEY", "hello").is_ok());
    }

    #[test]
    fn test_insert_exceeds_max_secrets() {
        let quota = Quota { max_secrets: 3, ..Quota::default() };
        let secrets = make_secrets(3);
        let err = check_insert(&quota, &secrets, "EXTRA", "val").unwrap_err();
        assert_eq!(err, QuotaError::TooManySecrets { limit: 3, current: 3 });
    }

    #[test]
    fn test_update_existing_key_does_not_count_as_new() {
        let quota = Quota { max_secrets: 2, ..Quota::default() };
        let mut secrets = make_secrets(2);
        // KEY_0 already exists — updating it should be allowed.
        secrets.insert("KEY_0".to_string(), "old".to_string());
        assert!(check_insert(&quota, &secrets, "KEY_0", "new_value").is_ok());
    }

    #[test]
    fn test_key_too_long() {
        let quota = Quota { max_key_length: 8, ..Quota::default() };
        let secrets = HashMap::new();
        let err = check_insert(&quota, &secrets, "TOOLONGKEY", "v").unwrap_err();
        assert_eq!(err, QuotaError::KeyTooLong { limit: 8, actual: 10 });
    }

    #[test]
    fn test_value_too_large() {
        let quota = Quota { max_value_bytes: 10, ..Quota::default() };
        let secrets = HashMap::new();
        let big = "x".repeat(11);
        let err = check_insert(&quota, &secrets, "KEY", &big).unwrap_err();
        assert_eq!(err, QuotaError::ValueTooLarge { limit: 10, actual: 11 });
    }

    #[test]
    fn test_usage_summary_format() {
        let quota = Quota { max_secrets: 100, max_key_length: 64, max_value_bytes: 1024 };
        let secrets = make_secrets(7);
        let summary = usage_summary(&quota, &secrets);
        assert!(summary.contains("7/100"));
        assert!(summary.contains("64 chars"));
        assert!(summary.contains("1024 bytes"));
    }

    #[test]
    fn test_quota_error_display() {
        let e = QuotaError::TooManySecrets { limit: 10, current: 10 };
        assert!(e.to_string().contains("quota exceeded"));
        let e2 = QuotaError::KeyTooLong { limit: 5, actual: 9 };
        assert!(e2.to_string().contains("key too long"));
        let e3 = QuotaError::ValueTooLarge { limit: 100, actual: 200 };
        assert!(e3.to_string().contains("value too large"));
    }
}
