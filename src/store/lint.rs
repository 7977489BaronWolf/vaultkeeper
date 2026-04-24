use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum LintLevel {
    Warning,
    Error,
}

#[derive(Debug)]
pub struct LintIssue {
    pub key: String,
    pub message: String,
    pub level: LintLevel,
}

pub fn lint_secrets(secrets: &HashMap<String, String>) -> Vec<LintIssue> {
    let mut issues = Vec::new();

    for (key, value) in secrets {
        // Check for empty values
        if value.trim().is_empty() {
            issues.push(LintIssue {
                key: key.clone(),
                message: "Value is empty".to_string(),
                level: LintLevel::Warning,
            });
        }

        // Check key naming convention (should be UPPER_SNAKE_CASE)
        if key != &key.to_uppercase() {
            issues.push(LintIssue {
                key: key.clone(),
                message: "Key should be UPPER_SNAKE_CASE".to_string(),
                level: LintLevel::Warning,
            });
        }

        // Check for keys with spaces
        if key.contains(' ') {
            issues.push(LintIssue {
                key: key.clone(),
                message: "Key contains spaces".to_string(),
                level: LintLevel::Error,
            });
        }

        // Check for potential plaintext secrets (very short values for sensitive keys)
        let sensitive_patterns = ["password", "secret", "token", "key", "api"];
        let key_lower = key.to_lowercase();
        if sensitive_patterns.iter().any(|p| key_lower.contains(p)) && value.len() < 8 {
            issues.push(LintIssue {
                key: key.clone(),
                message: "Sensitive key has suspiciously short value".to_string(),
                level: LintLevel::Warning,
            });
        }
    }

    issues
}

/// Returns only the issues matching the given lint level.
pub fn filter_issues_by_level(issues: &[LintIssue], level: LintLevel) -> Vec<&LintIssue> {
    issues.iter().filter(|i| i.level == level).collect()
}
