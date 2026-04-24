use std::collections::HashMap;

/// Maximum number of secrets allowed per namespace by default.
const DEFAULT_MAX_SECRETS: usize = 500;

/// Quota configuration for a vault namespace.
#[derive(Debug, Clone, PartialEq)]
pub struct Quota {
    pub max_secrets: usize,
    pub max_key_length: usize,
    pub max_value_bytes: usize,
}

impl Default for Quota {
    fn default() -> Self {
        Self {
            max_secrets: DEFAULT_MAX_SECRETS,
            max_key_length: 128,
            max_value_bytes: 65_536, // 64 KiB
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum QuotaError {
    TooManySecrets { limit: usize, current: usize },
    KeyTooLong { limit: usize, actual: usize },
    ValueTooLarge { limit: usize, actual: usize },
}

impl std::fmt::Display for QuotaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuotaError::TooManySecrets { limit, current } => {
                write!(f, "quota exceeded: max {limit} secrets, currently at {current}")
            }
            QuotaError::KeyTooLong { limit, actual } => {
                write!(f, "key too long: max {limit} chars, got {actual}")
            }
            QuotaError::ValueTooLarge { limit, actual } => {
                write!(f, "value too large: max {limit} bytes, got {actual}")
            }
        }
    }
}

/// Check whether adding a new secret would violate quota constraints.
pub fn check_insert(
    quota: &Quota,
    secrets: &HashMap<String, String>,
    key: &str,
    value: &str,
) -> Result<(), QuotaError> {
    // Only count as new if the key doesn't already exist.
    let is_new = !secrets.contains_key(key);
    if is_new && secrets.len() >= quota.max_secrets {
        return Err(QuotaError::TooManySecrets {
            limit: quota.max_secrets,
            current: secrets.len(),
        });
    }
    if key.len() > quota.max_key_length {
        return Err(QuotaError::KeyTooLong {
            limit: quota.max_key_length,
            actual: key.len(),
        });
    }
    if value.len() > quota.max_value_bytes {
        return Err(QuotaError::ValueTooLarge {
            limit: quota.max_value_bytes,
            actual: value.len(),
        });
    }
    Ok(())
}

/// Return a human-readable summary of current usage vs quota.
pub fn usage_summary(quota: &Quota, secrets: &HashMap<String, String>) -> String {
    format!(
        "secrets: {}/{} | key limit: {} chars | value limit: {} bytes",
        secrets.len(),
        quota.max_secrets,
        quota.max_key_length,
        quota.max_value_bytes,
    )
}
