use std::collections::HashMap;

/// Renders a template string by substituting `{{KEY}}` placeholders
/// with values from the provided env map.
pub fn render_template(template: &str, vars: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

/// Returns a list of placeholder keys found in the template.
pub fn extract_placeholders(template: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut remaining = template;
    while let Some(start) = remaining.find("{{") {
        let after_open = &remaining[start + 2..];
        if let Some(end) = after_open.find("}}") {
            let key = after_open[..end].trim().to_string();
            if !key.is_empty() && !keys.contains(&key) {
                keys.push(key);
            }
            remaining = &after_open[end + 2..];
        } else {
            break;
        }
    }
    keys
}

/// Checks whether all placeholders in the template have corresponding vars.
pub fn validate_template(template: &str, vars: &HashMap<String, String>) -> Vec<String> {
    extract_placeholders(template)
        .into_iter()
        .filter(|k| !vars.contains_key(k))
        .collect()
}
