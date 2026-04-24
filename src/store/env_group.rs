use std::collections::HashMap;

/// A named group of environment variable keys.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvGroup {
    pub name: String,
    pub keys: Vec<String>,
}

impl EnvGroup {
    pub fn new(name: impl Into<String>, keys: Vec<String>) -> Self {
        Self {
            name: name.into(),
            keys,
        }
    }
}

/// Parse env groups from a simple INI-style string.
/// Format:
///   [group_name]
///   KEY1
///   KEY2
pub fn parse_groups(input: &str) -> Vec<EnvGroup> {
    let mut groups: Vec<EnvGroup> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_keys: Vec<String> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            if let Some(name) = current_name.take() {
                groups.push(EnvGroup::new(name, current_keys.clone()));
                current_keys.clear();
            }
            current_name = Some(line[1..line.len() - 1].to_string());
        } else if current_name.is_some() {
            current_keys.push(line.to_string());
        }
    }
    if let Some(name) = current_name {
        groups.push(EnvGroup::new(name, current_keys));
    }
    groups
}

/// Filter a secrets map to only include keys belonging to a group.
pub fn filter_by_group<'a>(
    secrets: &'a HashMap<String, String>,
    group: &EnvGroup,
) -> HashMap<&'a str, &'a str> {
    secrets
        .iter()
        .filter(|(k, _)| group.keys.contains(k))
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect()
}

/// List keys present in the group but missing from secrets.
pub fn missing_keys(secrets: &HashMap<String, String>, group: &EnvGroup) -> Vec<String> {
    group
        .keys
        .iter()
        .filter(|k| !secrets.contains_key(*k))
        .cloned()
        .collect()
}
