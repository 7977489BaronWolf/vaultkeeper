use crate::store::scope::{list_all_scopes, resolve_with_parent, Scope};
use std::collections::HashMap;

pub fn cmd_scope_new(name: &str, parent: Option<&str>, scopes: &mut HashMap<String, Scope>) -> Result<(), String> {
    if scopes.contains_key(name) {
        return Err(format!("scope '{}' already exists", name));
    }
    if let Some(p) = parent {
        if !scopes.contains_key(p) {
            return Err(format!("parent scope '{}' not found", p));
        }
    }
    scopes.insert(name.to_string(), Scope::new(name, parent));
    println!("Created scope '{}'{}.", name, parent.map(|p| format!(" (parent: {})", p)).unwrap_or_default());
    Ok(())
}

pub fn cmd_scope_set(scope_name: &str, key: &str, value: &str, scopes: &mut HashMap<String, Scope>) -> Result<(), String> {
    let scope = scopes.get_mut(scope_name).ok_or_else(|| format!("scope '{}' not found", scope_name))?;
    scope.set(key, value);
    println!("Set '{}' in scope '{}'.", key, scope_name);
    Ok(())
}

pub fn cmd_scope_get(scope_name: &str, key: &str, scopes: &HashMap<String, Scope>) -> Result<(), String> {
    let scope = scopes.get(scope_name).ok_or_else(|| format!("scope '{}' not found", scope_name))?;
    match resolve_with_parent(key, scope, scopes) {
        Some(val) => {
            println!("{}", val);
            Ok(())
        }
        None => Err(format!("key '{}' not found in scope '{}' or its parents", key, scope_name)),
    }
}

pub fn cmd_scope_list(scopes: &HashMap<String, Scope>) {
    let names = list_all_scopes(scopes);
    if names.is_empty() {
        println!("No scopes defined.");
    } else {
        for name in &names {
            let scope = &scopes[name];
            let parent_info = scope.parent.as_deref().map(|p| format!(" -> {}", p)).unwrap_or_default();
            println!("  {}{} ({} key(s))", name, parent_info, scope.keys().len());
        }
    }
}

pub fn cmd_scope_delete(name: &str, scopes: &mut HashMap<String, Scope>) -> Result<(), String> {
    let has_children = scopes.values().any(|s| s.parent.as_deref() == Some(name));
    if has_children {
        return Err(format!("cannot delete scope '{}': it has child scopes", name));
    }
    if scopes.remove(name).is_some() {
        println!("Deleted scope '{}'.", name);
        Ok(())
    } else {
        Err(format!("scope '{}' not found", name))
    }
}
