use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the semantic type of a secret value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretType {
    Plain,
    ApiKey,
    Password,
    Token,
    ConnectionString,
    Certificate,
    PrivateKey,
    Url,
    Custom(String),
}

impl fmt::Display for SecretType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretType::Plain => write!(f, "plain"),
            SecretType::ApiKey => write!(f, "api_key"),
            SecretType::Password => write!(f, "password"),
            SecretType::Token => write!(f, "token"),
            SecretType::ConnectionString => write!(f, "connection_string"),
            SecretType::Certificate => write!(f, "certificate"),
            SecretType::PrivateKey => write!(f, "private_key"),
            SecretType::Url => write!(f, "url"),
            SecretType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

impl std::str::FromStr for SecretType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(SecretType::Plain),
            "api_key" => Ok(SecretType::ApiKey),
            "password" => Ok(SecretType::Password),
            "token" => Ok(SecretType::Token),
            "connection_string" => Ok(SecretType::ConnectionString),
            "certificate" => Ok(SecretType::Certificate),
            "private_key" => Ok(SecretType::PrivateKey),
            "url" => Ok(SecretType::Url),
            s if s.starts_with("custom:") => {
                Ok(SecretType::Custom(s[7..].to_string()))
            }
            other => Err(format!("Unknown secret type: '{}'", other)),
        }
    }
}

/// Returns a list of all built-in secret type names.
pub fn builtin_types() -> Vec<&'static str> {
    vec![
        "plain",
        "api_key",
        "password",
        "token",
        "connection_string",
        "certificate",
        "private_key",
        "url",
    ]
}

/// Annotates a key-value store entry with a secret type.
pub fn annotate(store: &mut std::collections::HashMap<String, String>, key: &str, secret_type: &SecretType) {
    let meta_key = format!("__type__{}", key);
    store.insert(meta_key, secret_type.to_string());
}

/// Reads the secret type annotation for a key from the store.
pub fn get_type(store: &std::collections::HashMap<String, String>, key: &str) -> SecretType {
    let meta_key = format!("__type__{}", key);
    store
        .get(&meta_key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(SecretType::Plain)
}
