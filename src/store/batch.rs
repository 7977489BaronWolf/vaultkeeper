use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BatchOp {
    Set { key: String, value: String },
    Delete { key: String },
    Unset { key: String },
}

#[derive(Debug, Clone)]
pub struct BatchResult {
    pub succeeded: Vec<String>,
    pub failed: Vec<(String, String)>,
}

impl BatchResult {
    pub fn new() -> Self {
        Self {
            succeeded: Vec::new(),
            failed: Vec::new(),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.failed.is_empty()
    }

    /// Returns the total number of operations processed (succeeded + failed).
    pub fn total(&self) -> usize {
        self.succeeded.len() + self.failed.len()
    }
}

pub fn apply_batch(
    store: &mut HashMap<String, String>,
    ops: Vec<BatchOp>,
) -> BatchResult {
    let mut result = BatchResult::new();

    for op in ops {
        match op {
            BatchOp::Set { key, value } => {
                if key.trim().is_empty() {
                    result.failed.push((key, "key must not be empty".into()));
                } else {
                    store.insert(key.clone(), value);
                    result.succeeded.push(key);
                }
            }
            BatchOp::Delete { key } | BatchOp::Unset { key } => {
                if store.remove(&key).is_some() {
                    result.succeeded.push(key);
                } else {
                    result
                        .failed
                        .push((key.clone(), format!("key '{}' not found", key)));
                }
            }
        }
    }

    result
}

pub fn parse_batch_ops(input: &str) -> Result<Vec<BatchOp>, String> {
    let mut ops = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("SET ") {
            if let Some((k, v)) = rest.split_once('=') {
                let key = k.trim().to_string();
                if key.is_empty() {
                    return Err(format!("line {}: SET key must not be empty", i + 1));
                }
                ops.push(BatchOp::Set {
                    key,
                    value: v.to_string(),
                });
            } else {
                return Err(format!("line {}: invalid SET syntax", i + 1));
            }
        } else if let Some(key) = line.strip_prefix("DELETE ") {
            let key = key.trim().to_string();
            if key.is_empty() {
                return Err(format!("line {}: DELETE key must not be empty", i + 1));
            }
            ops.push(BatchOp::Delete { key });
        } else if let Some(key) = line.strip_prefix("UNSET ") {
            let key = key.trim().to_string();
            if key.is_empty() {
                return Err(format!("line {}: UNSET key must not be empty", i + 1));
            }
            ops.push(BatchOp::Unset { key });
        } else {
            return Err(format!("line {}: unknown operation", i + 1));
        }
    }
    Ok(ops)
}
