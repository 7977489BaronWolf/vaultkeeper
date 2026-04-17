#[cfg(test)]
mod tests {
    use super::super::lint::{lint_secrets, LintLevel};
    use std::collections::HashMap;

    fn make_secrets(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_no_issues_for_valid_secrets() {
        let secrets = make_secrets(&[("DATABASE_URL", "postgres://localhost/mydb")]);
        let issues = lint_secrets(&secrets);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_empty_value_warning() {
        let secrets = make_secrets(&[("MY_KEY", "")]);
        let issues = lint_secrets(&secrets);
        assert!(issues.iter().any(|i| i.key == "MY_KEY" && i.level == LintLevel::Warning && i.message.contains("empty")));
    }

    #[test]
    fn test_lowercase_key_warning() {
        let secrets = make_secrets(&[("my_key", "somevalue")]);
        let issues = lint_secrets(&secrets);
        assert!(issues.iter().any(|i| i.key == "my_key" && i.message.contains("UPPER_SNAKE_CASE")));
    }

    #[test]
    fn test_key_with_spaces_error() {
        let secrets = make_secrets(&[("MY KEY", "value")]);
        let issues = lint_secrets(&secrets);
        assert!(issues.iter().any(|i| i.key == "MY KEY" && i.level == LintLevel::Error));
    }

    #[test]
    fn test_short_sensitive_value_warning() {
        let secrets = make_secrets(&[("API_TOKEN", "abc")]);
        let issues = lint_secrets(&secrets);
        assert!(issues.iter().any(|i| i.key == "API_TOKEN" && i.message.contains("short")));
    }

    #[test]
    fn test_sensitive_key_long_value_no_warning() {
        let secrets = make_secrets(&[("API_TOKEN", "supersecretlongtoken123")]);
        let issues = lint_secrets(&secrets);
        assert!(!issues.iter().any(|i| i.message.contains("short")));
    }

    #[test]
    fn test_multiple_issues_same_key() {
        let secrets = make_secrets(&[("api token", "")]);
        let issues = lint_secrets(&secrets);
        // spaces error + empty warning
        assert!(issues.len() >= 2);
    }
}
