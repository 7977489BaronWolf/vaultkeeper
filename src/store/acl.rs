use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Delete,
}

#[derive(Debug, Clone, Default)]
pub struct Acl {
    /// Maps key patterns (glob-style) to allowed permissions per identity
    entries: HashMap<String, HashMap<String, HashSet<String>>>,
}

impl Acl {
    pub fn new() -> Self {
        Self::default()
    }

    /// Grant a permission to an identity for a key pattern.
    pub fn grant(&mut self, pattern: &str, identity: &str, permission: Permission) {
        let perm_str = permission_to_str(&permission);
        self.entries
            .entry(pattern.to_string())
            .or_default()
            .entry(identity.to_string())
            .or_default()
            .insert(perm_str.to_string());
    }

    /// Revoke a permission from an identity for a key pattern.
    pub fn revoke(&mut self, pattern: &str, identity: &str, permission: &Permission) {
        let perm_str = permission_to_str(permission);
        if let Some(id_map) = self.entries.get_mut(pattern) {
            if let Some(perms) = id_map.get_mut(identity) {
                perms.remove(perm_str);
            }
        }
    }

    /// Check whether an identity has a specific permission for a key.
    pub fn is_allowed(&self, key: &str, identity: &str, permission: &Permission) -> bool {
        let perm_str = permission_to_str(permission);
        for (pattern, id_map) in &self.entries {
            if glob_match(pattern, key) {
                if let Some(perms) = id_map.get(identity) {
                    if perms.contains(perm_str) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// List all entries as (pattern, identity, permission) triples.
    pub fn list(&self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        for (pattern, id_map) in &self.entries {
            for (identity, perms) in id_map {
                for perm in perms {
                    result.push((pattern.clone(), identity.clone(), perm.clone()));
                }
            }
        }
        result.sort();
        result
    }
}

fn permission_to_str(p: &Permission) -> &'static str {
    match p {
        Permission::Read => "read",
        Permission::Write => "write",
        Permission::Delete => "delete",
    }
}

/// Minimal glob matching: supports `*` as wildcard segment.
fn glob_match(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == value;
    }
    let parts: Vec<&str> = pattern.splitn(2, '*').collect();
    let prefix = parts[0];
    let suffix = parts[1];
    value.starts_with(prefix) && value.ends_with(suffix)
        && value.len() >= prefix.len() + suffix.len()
}
