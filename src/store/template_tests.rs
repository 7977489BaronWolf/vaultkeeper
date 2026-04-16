#[cfg(test)]
mod tests {
    use super::super::template::*;
    use std::collections::HashMap;

    fn make_vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_render_simple() {
        let vars = make_vars(&[("NAME", "world")]);
        assert_eq!(render_template("Hello, {{NAME}}!", &vars), "Hello, world!");
    }

    #[test]
    fn test_render_multiple() {
        let vars = make_vars(&[("HOST", "localhost"), ("PORT", "5432")]);
        let out = render_template("{{HOST}}:{{PORT}}", &vars);
        assert_eq!(out, "localhost:5432");
    }

    #[test]
    fn test_render_missing_key_unchanged() {
        let vars = make_vars(&[]);
        let out = render_template("{{MISSING}}", &vars);
        assert_eq!(out, "{{MISSING}}");
    }

    #[test]
    fn test_extract_placeholders() {
        let keys = extract_placeholders("{{A}} and {{B}} and {{A}}");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"A".to_string()));
        assert!(keys.contains(&"B".to_string()));
    }

    #[test]
    fn test_extract_empty() {
        assert!(extract_placeholders("no placeholders here").is_empty());
    }

    #[test]
    fn test_validate_template_all_present() {
        let vars = make_vars(&[("X", "1"), ("Y", "2")]);
        let missing = validate_template("{{X}} {{Y}}", &vars);
        assert!(missing.is_empty());
    }

    #[test]
    fn test_validate_template_missing() {
        let vars = make_vars(&[("X", "1")]);
        let missing = validate_template("{{X}} {{Z}}", &vars);
        assert_eq!(missing, vec!["Z".to_string()]);
    }
}
