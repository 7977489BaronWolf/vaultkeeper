use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaType {
    String,
    Number,
    Boolean,
    Url,
    Email,
}

impl std::fmt::Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::String => write!(f, "string"),
            SchemaType::Number => write!(f, "number"),
            SchemaType::Boolean => write!(f, "boolean"),
            SchemaType::Url => write!(f, "url"),
            SchemaType::Email => write!(f, "email"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub field_type: SchemaType,
    pub required: bool,
    pub description: Option<String>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Schema {
    pub fields: HashMap<String, SchemaField>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_field(&mut self, key: String, field: SchemaField) {
        self.fields.insert(key, field);
    }

    pub fn remove_field(&mut self, key: &str) -> bool {
        self.fields.remove(key).is_some()
    }

    pub fn validate_value(&self, key: &str, value: &str) -> Result<(), String> {
        let field = match self.fields.get(key) {
            Some(f) => f,
            None => return Ok(()),
        };
        match field.field_type {
            SchemaType::Number => {
                value.parse::<f64>().map_err(|_| format!("'{}' must be a number", key))?;
            }
            SchemaType::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "false" | "1" | "0" | "yes" | "no" => {}
                    _ => return Err(format!("'{}' must be a boolean", key)),
                }
            }
            SchemaType::Url => {
                if !value.starts_with("http://") && !value.starts_with("https://") {
                    return Err(format!("'{}' must be a valid URL", key));
                }
            }
            SchemaType::Email => {
                if !value.contains('@') || !value.contains('.') {
                    return Err(format!("'{}' must be a valid email", key));
                }
            }
            SchemaType::String => {
                if let Some(pattern) = &field.pattern {
                    let re = regex::Regex::new(pattern)
                        .map_err(|e| format!("Invalid pattern for '{}': {}", key, e))?;
                    if !re.is_match(value) {
                        return Err(format!("'{}' does not match pattern '{}'", key, pattern));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn check_required(&self, keys: &[&str]) -> Vec<String> {
        self.fields
            .iter()
            .filter(|(_, f)| f.required)
            .filter(|(k, _)| !keys.contains(&k.as_str()))
            .map(|(k, _)| k.clone())
            .collect()
    }
}
