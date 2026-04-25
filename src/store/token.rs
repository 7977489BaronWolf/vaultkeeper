use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::store::audit::AuditLog;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub id: String,
    pub label: String,
    pub scopes: Vec<String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub last_used: Option<u64>,
}

impl Token {
    pub fn new(id: &str, label: &str, scopes: Vec<String>, expires_at: Option<u64>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Token {
            id: id.to_string(),
            label: label.to_string(),
            scopes,
            created_at: now,
            expires_at,
            last_used: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            return now > exp;
        }
        false
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == "*" || s == scope)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenStore {
    pub tokens: HashMap<String, Token>,
}

impl TokenStore {
    pub fn add(&mut self, token: Token) {
        self.tokens.insert(token.id.clone(), token);
    }

    pub fn remove(&mut self, id: &str) -> Option<Token> {
        self.tokens.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&Token> {
        self.tokens.get(id)
    }

    pub fn list_valid(&self) -> Vec<&Token> {
        self.tokens.values().filter(|t| !t.is_expired()).collect()
    }

    pub fn purge_expired(&mut self) -> usize {
        let expired: Vec<String> = self.tokens
            .iter()
            .filter(|(_, t)| t.is_expired())
            .map(|(id, _)| id.clone())
            .collect();
        let count = expired.len();
        for id in expired {
            self.tokens.remove(&id);
        }
        count
    }
}
