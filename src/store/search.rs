use std::collections::HashMap;

/// Search secrets by key pattern or value substring.
pub fn search_by_key(secrets: &HashMap<String, String>, pattern: &str) -> Vec<String> {
    let pattern_lower = pattern.to_lowercase();
    let mut matches: Vec<String> = secrets
        .keys()
        .filter(|k| k.to_lowercase().contains(&pattern_lower))
        .cloned()
        .collect();
    matches.sort();
    matches
}

/// Search secrets whose values contain the given substring.
pub fn search_by_value(secrets: &HashMap<String, String>, substring: &str) -> Vec<String> {
    let sub_lower = substring.to_lowercase();
    let mut matches: Vec<String> = secrets
        .iter()
        .filter(|(_, v)| v.to_lowercase().contains(&sub_lower))
        .map(|(k, _)| k.clone())
        .collect();
    matches.sort();
    matches
}

/// Search secrets by key pattern and return key=value pairs.
pub fn search_entries(
    secrets: &HashMap<String, String>,
    pattern: &str,
    by_value: bool,
) -> Vec<(String, String)> {
    let keys = if by_value {
        search_by_value(secrets, pattern)
    } else {
        search_by_key(secrets, pattern)
    };
    keys.into_iter()
        .filter_map(|k| secrets.get(&k).map(|v| (k, v.clone())))
        .collect()
}
