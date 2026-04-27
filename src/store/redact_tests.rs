#[cfg(test)]
mod tests {
    use super::super::redact::*;
    use std::collections::HashSet;

    #[test]
    fn test_redact_value_short() {
        assert_eq!(redact_value("abc"), "****");
        assert_eq!(redact_value(""), "****");
        assert_eq!(redact_value("abcd"), "****");
    }

    #[test]
    fn test_redact_value_long() {
        let result = redact_value("supersecret123");
        assert!(result.starts_with("su"));
        assert!(result.ends_with("23"));
        assert!(result.contains("****"));
    }

    #[test]
    fn test_is_sensitive_key_positive() {
        assert!(is_sensitive_key("DB_PASSWORD"));
        assert!(is_sensitive_key("API_TOKEN"));
        assert!(is_sensitive_key("AWS_SECRET_ACCESS_KEY"));
        assert!(is_sensitive_key("GITHUB_AUTH_TOKEN"));
        assert!(is_sensitive_key("PRIVATE_KEY_PATH"));
    }

    #[test]
    fn test_is_sensitive_key_negative() {
        assert!(!is_sensitive_key("DATABASE_HOST"));
        assert!(!is_sensitive_key("APP_NAME"));
        assert!(!is_sensitive_key("PORT"));
        assert!(!is_sensitive_key("LOG_LEVEL"));
    }

    #[test]
    fn test_redact_env() {
        let env = vec![
            ("APP_NAME".to_string(), "myapp".to_string()),
            ("DB_PASSWORD".to_string(), "hunter2hunter".to_string()),
            ("PORT".to_string(), "8080".to_string()),
        ];
        let redacted = redact_env(&env);
        assert_eq!(redacted[0].1, "myapp");
        assert_ne!(redacted[1].1, "hunter2hunter");
        assert!(redacted[1].1.contains("****"));
        assert_eq!(redacted[2].1, "8080");
    }

    #[test]
    fn test_redact_keys_forced() {
        let env = vec![
            ("MY_CUSTOM_VAR".to_string(), "sensitive_data_here".to_string()),
            ("NORMAL_VAR".to_string(), "visible".to_string()),
        ];
        let mut forced = HashSet::new();
        forced.insert("MY_CUSTOM_VAR".to_string());
        let redacted = redact_keys(&env, &forced);
        assert!(redacted[0].1.contains("****"));
        assert_eq!(redacted[1].1, "visible");
    }
}
