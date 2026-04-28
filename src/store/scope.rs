use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub name: String,
    pub parent: Option<String>,
    pub vars: HashMap<String, String>,
}

impl Scope {
    pub fn new(name: &str, parent: Option<&str>) -> Self {
        Scope {
            name: name.to_string(),
            parent: parent.map(|s| s.to_string()),
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.vars.remove(key).is_some()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.vars.keys().collect()
    }
}

pub fn resolve_with_parent<'a>(
    key: &str,
    scope: &'a Scope,
    all_scopes: &'a HashMap<String, Scope>,
) -> Option<&'a String> {
    if let Some(val) = scope.get(key) {
        return Some(val);
    }
    if let Some(parent_name) = &scope.parent {
        if let Some(parent_scope) = all_scopes.get(parent_name) {
            return resolve_with_parent(key, parent_scope, all_scopes);
        }
    }
    None
}

pub fn list_all_scopes(all_scopes: &HashMap<String, Scope>) -> Vec<String> {
    let mut names: Vec<String> = all_scopes.keys().cloned().collect();
    names.sort();
    names
}
