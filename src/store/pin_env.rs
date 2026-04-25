use std::collections::HashMap;

/// Represents a pinned environment — a named snapshot of key-value pairs
/// that can be restored or compared against the live store.
#[derive(Debug, Clone, PartialEq)]
pub struct PinnedEnv {
    pub name: String,
    pub vars: HashMap<String, String>,
}

impl PinnedEnv {
    pub fn new(name: impl Into<String>, vars: HashMap<String, String>) -> Self {
        Self {
            name: name.into(),
            vars,
        }
    }
}

/// Pin a named environment from a flat list of KEY=VALUE pairs.
pub fn pin_env(
    name: &str,
    entries: &[(String, String)],
) -> Result<PinnedEnv, String> {
    if name.trim().is_empty() {
        return Err("Pin name must not be empty".to_string());
    }
    let vars: HashMap<String, String> = entries
        .iter()
        .cloned()
        .collect();
    Ok(PinnedEnv::new(name, vars))
}

/// Compare a pinned env against a current set of entries.
/// Returns (added, removed, changed) key lists.
pub fn diff_pinned(
    pinned: &PinnedEnv,
    current: &[(String, String)],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let current_map: HashMap<&str, &str> =
        current.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (k, v) in &pinned.vars {
        match current_map.get(k.as_str()) {
            Some(cv) if *cv != v.as_str() => changed.push(k.clone()),
            None => removed.push(k.clone()),
            _ => {}
        }
    }

    for (k, _) in current {
        if !pinned.vars.contains_key(k) {
            added.push(k.clone());
        }
    }

    added.sort();
    removed.sort();
    changed.sort();
    (added, removed, changed)
}
