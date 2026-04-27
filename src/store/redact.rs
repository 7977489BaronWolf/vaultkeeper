use std::collections::HashSet;

/// Patterns that indicate a value should be redacted in output
const SENSITIVE_PATTERNS: &[&str] = &[
    "password", "passwd", "secret", "token", "api_key", "apikey",
    "private_key", "auth", "credential", "cert", "passphrase",
];

/// Redacts a secret value, showing only the first and last 2 chars
pub fn redact_value(value: &str) -> String {
    let len = value.len();
    if len <= 4 {
        return "****".to_string();
    }
    format!("{}****{}", &value[..2], &value[len - 2..])
}

/// Returns true if the key name suggests its value is sensitive
pub fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_lowercase();
    SENSITIVE_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Redacts values from a map of env vars based on key sensitivity
pub fn redact_env(env: &[(String, String)]) -> Vec<(String, String)> {
    env.iter()
        .map(|(k, v)| {
            if is_sensitive_key(k) {
                (k.clone(), redact_value(v))
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

/// Redacts all values for a given set of explicitly marked keys
pub fn redact_keys<'a>(
    env: &'a [(String, String)],
    forced: &HashSet<String>,
) -> Vec<(String, String)> {
    env.iter()
        .map(|(k, v)| {
            if forced.contains(k) || is_sensitive_key(k) {
                (k.clone(), redact_value(v))
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}
