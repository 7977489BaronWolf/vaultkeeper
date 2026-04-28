use std::collections::HashMap;

/// A namespace groups secrets under a logical prefix, enabling
/// multi-environment or multi-service secret organization.
#[derive(Debug, Clone, PartialEq)]
pub struct Namespace {
    pub name: String,
    pub description: Option<String>,
}

impl Namespace {
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Namespace {
            name: name.to_string(),
            description: description.map(|d| d.to_string()),
        }
    }

    /// Returns true if the namespace name is valid (alphanumeric + hyphens/underscores).
    pub fn is_valid_name(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

/// Manages a registry of namespaces and maps secret keys to their namespace.
pub struct NamespaceStore {
    pub namespaces: HashMap<String, Namespace>,
    /// Maps secret key -> namespace name
    pub assignments: HashMap<String, String>,
}

impl NamespaceStore {
    pub fn new() -> Self {
        NamespaceStore {
            namespaces: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    pub fn add_namespace(&mut self, ns: Namespace) -> Result<(), String> {
        if !Namespace::is_valid_name(&ns.name) {
            return Err(format!("Invalid namespace name: '{}'", ns.name));
        }
        if self.namespaces.contains_key(&ns.name) {
            return Err(format!("Namespace '{}' already exists", ns.name));
        }
        self.namespaces.insert(ns.name.clone(), ns);
        Ok(())
    }

    pub fn remove_namespace(&mut self, name: &str) -> Result<(), String> {
        if !self.namespaces.contains_key(name) {
            return Err(format!("Namespace '{}' not found", name));
        }
        self.namespaces.remove(name);
        self.assignments.retain(|_, v| v != name);
        Ok(())
    }

    pub fn assign(&mut self, key: &str, namespace: &str) -> Result<(), String> {
        if !self.namespaces.contains_key(namespace) {
            return Err(format!("Namespace '{}' does not exist", namespace));
        }
        self.assignments.insert(key.to_string(), namespace.to_string());
        Ok(())
    }

    /// Removes the namespace assignment for the given key, if one exists.
    /// Returns `true` if an assignment was removed, `false` if the key had no assignment.
    pub fn unassign(&mut self, key: &str) -> bool {
        self.assignments.remove(key).is_some()
    }

    pub fn get_namespace_for_key(&self, key: &str) -> Option<&Namespace> {
        self.assignments
            .get(key)
            .and_then(|ns_name| self.namespaces.get(ns_name))
    }

    pub fn keys_in_namespace(&self, namespace: &str) -> Vec<&str> {
        self.assignments
            .iter()
            .filter(|(_, v)| v.as_str() == namespace)
            .map(|(k, _)| k.as_str())
            .collect()
    }

    pub fn list_namespaces(&self) -> Vec<&Namespace> {
        let mut ns: Vec<&Namespace> = self.namespaces.values().collect();
        ns.sort_by(|a, b| a.name.cmp(&b.name));
        ns
    }
}
