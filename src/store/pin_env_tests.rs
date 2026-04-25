#[cfg(test)]
mod tests {
    use super::super::pin_env::{diff_pinned, pin_env};
    use std::collections::HashMap;

    fn entries(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_pin_env_basic() {
        let e = entries(&[("FOO", "bar"), ("BAZ", "qux")]);
        let pinned = pin_env("mypin", &e).unwrap();
        assert_eq!(pinned.name, "mypin");
        assert_eq!(pinned.vars.get("FOO").unwrap(), "bar");
        assert_eq!(pinned.vars.get("BAZ").unwrap(), "qux");
    }

    #[test]
    fn test_pin_env_empty_name_fails() {
        let e = entries(&[("FOO", "bar")]);
        let result = pin_env("", &e);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_pin_env_whitespace_name_fails() {
        let e = entries(&[("FOO", "bar")]);
        let result = pin_env("   ", &e);
        assert!(result.is_err());
    }

    #[test]
    fn test_diff_no_changes() {
        let e = entries(&[("FOO", "bar"), ("BAZ", "qux")]);
        let pinned = pin_env("snap", &e).unwrap();
        let (added, removed, changed) = diff_pinned(&pinned, &e);
        assert!(added.is_empty());
        assert!(removed.is_empty());
        assert!(changed.is_empty());
    }

    #[test]
    fn test_diff_added_key() {
        let e = entries(&[("FOO", "bar")]);
        let pinned = pin_env("snap", &e).unwrap();
        let current = entries(&[("FOO", "bar"), ("NEW", "val")]);
        let (added, removed, changed) = diff_pinned(&pinned, &current);
        assert_eq!(added, vec!["NEW"]);
        assert!(removed.is_empty());
        assert!(changed.is_empty());
    }

    #[test]
    fn test_diff_removed_key() {
        let e = entries(&[("FOO", "bar"), ("GONE", "bye")]);
        let pinned = pin_env("snap", &e).unwrap();
        let current = entries(&[("FOO", "bar")]);
        let (added, removed, changed) = diff_pinned(&pinned, &current);
        assert!(added.is_empty());
        assert_eq!(removed, vec!["GONE"]);
        assert!(changed.is_empty());
    }

    #[test]
    fn test_diff_changed_key() {
        let e = entries(&[("FOO", "old")]);
        let pinned = pin_env("snap", &e).unwrap();
        let current = entries(&[("FOO", "new")]);
        let (added, removed, changed) = diff_pinned(&pinned, &current);
        assert!(added.is_empty());
        assert!(removed.is_empty());
        assert_eq!(changed, vec!["FOO"]);
    }
}
