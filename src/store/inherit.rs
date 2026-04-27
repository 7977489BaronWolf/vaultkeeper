use std::collections::HashMap;

/// Resolves inherited values from a parent environment into a child environment.
/// Keys present in the child take precedence over the parent.
pub fn resolve_inheritance(
    parent: &HashMap<String, String>,
    child: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = parent.clone();
    for (k, v) in child {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

/// Returns the list of keys that the child overrides from the parent.
pub fn overridden_keys(
    parent: &HashMap<String, String>,
    child: &HashMap<String, String>,
) -> Vec<String> {
    child
        .keys()
        .filter(|k| parent.contains_key(*k))
        .cloned()
        .collect()
}

/// Returns keys inherited from parent that are not present in child.
pub fn inherited_keys(
    parent: &HashMap<String, String>,
    child: &HashMap<String, String>,
) -> Vec<String> {
    parent
        .keys()
        .filter(|k| !child.contains_key(*k))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "inherit_tests.rs"]
mod tests;
