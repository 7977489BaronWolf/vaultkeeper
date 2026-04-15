use std::collections::HashMap;

/// Parses inline tags from an env file comment line.
/// Tags are expected in the format: `# @tags: tag1,tag2,tag3`
pub fn parse_tags_from_comment(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("# @tags:") {
        rest.split(',')
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect()
    } else {
        vec![]
    }
}

/// Extracts a map of key -> tags from a raw env file string.
/// Tags must appear on the line immediately before the key=value pair.
pub fn extract_tags(content: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut pending_tags: Vec<String> = vec![];

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("# @tags:") {
            pending_tags = parse_tags_from_comment(trimmed);
        } else if !trimmed.starts_with('#') && trimmed.contains('=') {
            if let Some(key) = trimmed.splitn(2, '=').next() {
                let key = key.trim().to_string();
                if !pending_tags.is_empty() {
                    map.insert(key, pending_tags.clone());
                    pending_tags.clear();
                }
            }
        } else {
            pending_tags.clear();
        }
    }

    map
}

/// Filters env key=value pairs by a given tag, returning only matching lines.
pub fn filter_by_tag<'a>(content: &'a str, tag: &str) -> Vec<&'a str> {
    let tag = tag.to_lowercase();
    let lines: Vec<&str> = content.lines().collect();
    let mut result = vec![];
    let mut pending_tags: Vec<String> = vec![];

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("# @tags:") {
            pending_tags = parse_tags_from_comment(trimmed);
        } else if !trimmed.starts_with('#') && trimmed.contains('=') {
            if pending_tags.contains(&tag) {
                result.push(*line);
            }
            pending_tags.clear();
        } else {
            pending_tags.clear();
        }
    }

    result
}
