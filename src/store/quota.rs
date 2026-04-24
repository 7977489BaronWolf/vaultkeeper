use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuotaConfig {
    pub max_keys: usize,
    pub max_value_bytes: usize,
    pub max_total_bytes: usize,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            max_keys: 500,
            max_value_bytes: 4096,
            max_total_bytes: 1_048_576, // 1 MiB
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuotaReport {
    pub key_count: usize,
    pub max_keys: usize,
    pub total_bytes: usize,
    pub max_total_bytes: usize,
    pub largest_key: Option<String>,
    pub largest_value_bytes: usize,
    pub violations: Vec<String>,
}

impl QuotaReport {
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}

pub fn check_quota(secrets: &HashMap<String, String>, config: &QuotaConfig) -> QuotaReport {
    let key_count = secrets.len();
    let mut total_bytes = 0usize;
    let mut largest_key: Option<String> = None;
    let mut largest_value_bytes = 0usize;
    let mut violations = Vec::new();

    for (k, v) in secrets {
        let vlen = v.len();
        total_bytes += k.len() + vlen;

        if vlen > largest_value_bytes {
            largest_value_bytes = vlen;
            largest_key = Some(k.clone());
        }

        if vlen > config.max_value_bytes {
            violations.push(format!(
                "key '{}' value exceeds max ({} > {} bytes)",
                k, vlen, config.max_value_bytes
            ));
        }
    }

    if key_count > config.max_keys {
        violations.push(format!(
            "key count exceeds max ({} > {})",
            key_count, config.max_keys
        ));
    }

    if total_bytes > config.max_total_bytes {
        violations.push(format!(
            "total size exceeds max ({} > {} bytes)",
            total_bytes, config.max_total_bytes
        ));
    }

    QuotaReport {
        key_count,
        max_keys: config.max_keys,
        total_bytes,
        max_total_bytes: config.max_total_bytes,
        largest_key,
        largest_value_bytes,
        violations,
    }
}
