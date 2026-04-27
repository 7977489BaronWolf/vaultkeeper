use super::*;
use std::collections::HashMap;

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn test_child_overrides_parent() {
    let parent = map(&[("DB_HOST", "localhost"), ("DB_PORT", "5432")]);
    let child = map(&[("DB_HOST", "prod.db.example.com")]);
    let result = resolve_inheritance(&parent, &child);
    assert_eq!(result.get("DB_HOST").unwrap(), "prod.db.example.com");
    assert_eq!(result.get("DB_PORT").unwrap(), "5432");
}

#[test]
fn test_child_adds_new_keys() {
    let parent = map(&[("FOO", "bar")]);
    let child = map(&[("BAZ", "qux")]);
    let result = resolve_inheritance(&parent, &child);
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("FOO").unwrap(), "bar");
    assert_eq!(result.get("BAZ").unwrap(), "qux");
}

#[test]
fn test_empty_child_returns_parent() {
    let parent = map(&[("KEY", "value")]);
    let child = map(&[]);
    let result = resolve_inheritance(&parent, &child);
    assert_eq!(result, parent);
}

#[test]
fn test_overridden_keys_detected() {
    let parent = map(&[("A", "1"), ("B", "2"), ("C", "3")]);
    let child = map(&[("A", "10"), ("D", "4")]);
    let mut keys = overridden_keys(&parent, &child);
    keys.sort();
    assert_eq!(keys, vec!["A"]);
}

#[test]
fn test_inherited_keys_detected() {
    let parent = map(&[("A", "1"), ("B", "2")]);
    let child = map(&[("A", "override")]);
    let mut keys = inherited_keys(&parent, &child);
    keys.sort();
    assert_eq!(keys, vec!["B"]);
}

#[test]
fn test_no_inheritance_when_child_has_all() {
    let parent = map(&[("X", "1")]);
    let child = map(&[("X", "2")]);
    let keys = inherited_keys(&parent, &child);
    assert!(keys.is_empty());
}
