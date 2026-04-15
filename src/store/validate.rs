use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    EmptyKey,
    InvalidKeyChar(String),
    EmptyValue,
    DuplicateKey(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyKey => write!(f, "Key must not be empty"),
            ValidationError::InvalidKeyChar(k) => {
                write!(f, "Key '{}' contains invalid characters (use A-Z, 0-9, _)", k)
            }
            ValidationError::EmptyValue => write!(f, "Value must not be empty"),
            ValidationError::DuplicateKey(k) => write!(f, "Duplicate key: '{}'", k),
        }
    }
}

pub fn validate_key(key: &str) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::EmptyKey);
    }
    let valid = key
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if !valid {
        return Err(ValidationError::InvalidKeyChar(key.to_string()));
    }
    Ok(())
}

pub fn validate_value(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::EmptyValue);
    }
    Ok(())
}

pub fn validate_no_duplicates(
    entries: &[(String, String)],
) -> Result<(), ValidationError> {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for (i, (key, _)) in entries.iter().enumerate() {
        if let Some(_prev) = seen.insert(key.as_str(), i) {
            return Err(ValidationError::DuplicateKey(key.clone()));
        }
    }
    Ok(())
}

pub fn validate_entry(key: &str, value: &str) -> Result<(), ValidationError> {
    validate_key(key)?;
    validate_value(value)?;
    Ok(())
}
