use crate::store::inherit::{inherited_keys, overridden_keys, resolve_inheritance};
use std::collections::HashMap;

/// Parses a simple KEY=VALUE env file content into a HashMap.
fn parse_env_map(content: &str) -> HashMap<String, String> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.trim().to_string();
            let val = parts.next().unwrap_or("").trim().to_string();
            Some((key, val))
        })
        .collect()
}

/// Renders a HashMap back to KEY=VALUE lines, sorted for determinism.
fn render_env_map(map: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = map.iter().collect();
    pairs.sort_by_key(|(k, _)| k.as_str());
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Merges parent env content into child env content, with child values taking priority.
/// Returns the merged content as a string.
pub fn handle_inherit(parent_content: &str, child_content: &str, verbose: bool) -> String {
    let parent = parse_env_map(parent_content);
    let child = parse_env_map(child_content);

    if verbose {
        let mut over = overridden_keys(&parent, &child);
        over.sort();
        let mut inh = inherited_keys(&parent, &child);
        inh.sort();
        if !inh.is_empty() {
            eprintln!("[inherit] inheriting: {}", inh.join(", "));
        }
        if !over.is_empty() {
            eprintln!("[inherit] overriding: {}", over.join(", "));
        }
    }

    let merged = resolve_inheritance(&parent, &child);
    render_env_map(&merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_inherit_merges_correctly() {
        let parent = "DB_HOST=localhost\nDB_PORT=5432\n";
        let child = "DB_HOST=prod.example.com\nAPP_ENV=production\n";
        let result = handle_inherit(parent, child, false);
        assert!(result.contains("DB_HOST=prod.example.com"));
        assert!(result.contains("DB_PORT=5432"));
        assert!(result.contains("APP_ENV=production"));
    }

    #[test]
    fn test_handle_inherit_ignores_comments() {
        let parent = "# comment\nFOO=bar\n";
        let child = "BAZ=qux\n";
        let result = handle_inherit(parent, child, false);
        assert!(result.contains("FOO=bar"));
        assert!(result.contains("BAZ=qux"));
        assert!(!result.contains("# comment"));
    }
}
