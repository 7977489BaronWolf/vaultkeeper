#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::store::merge::{merge_envs, MergeStrategy};

    fn base() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("FOO".into(), "foo_base".into());
        m.insert("BAR".into(), "bar_base".into());
        m
    }

    fn incoming() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("FOO".into(), "foo_new".into());
        m.insert("BAZ".into(), "baz_new".into());
        m
    }

    #[test]
    fn test_overwrite_strategy() {
        let mut base = base();
        let result = merge_envs(&mut base, &incoming(), &MergeStrategy::Overwrite);

        assert_eq!(base.get("FOO").unwrap(), "foo_new");
        assert_eq!(base.get("BAR").unwrap(), "bar_base");
        assert_eq!(base.get("BAZ").unwrap(), "baz_new");
        assert_eq!(result.overwritten, vec!["FOO"]);
        assert_eq!(result.added, vec!["BAZ"]);
        assert!(result.skipped.is_empty());
    }

    #[test]
    fn test_keep_base_strategy() {
        let mut base = base();
        let result = merge_envs(&mut base, &incoming(), &MergeStrategy::KeepBase);

        assert_eq!(base.get("FOO").unwrap(), "foo_base");
        assert_eq!(base.get("BAZ").unwrap(), "baz_new");
        assert_eq!(result.skipped, vec!["FOO"]);
        assert_eq!(result.added, vec!["BAZ"]);
        assert!(result.overwritten.is_empty());
    }

    #[test]
    fn test_add_only_strategy() {
        let mut base = base();
        let result = merge_envs(&mut base, &incoming(), &MergeStrategy::AddOnly);

        assert_eq!(base.get("FOO").unwrap(), "foo_base");
        assert_eq!(base.get("BAZ").unwrap(), "baz_new");
        assert_eq!(result.added, vec!["BAZ"]);
        assert_eq!(result.skipped, vec!["FOO"]);
        assert!(result.overwritten.is_empty());
    }

    #[test]
    fn test_merge_empty_incoming() {
        let mut base = base();
        let incoming: HashMap<String, String> = HashMap::new();
        let result = merge_envs(&mut base, &incoming, &MergeStrategy::Overwrite);

        assert_eq!(base.len(), 2);
        assert!(result.added.is_empty());
        assert!(result.overwritten.is_empty());
        assert!(result.skipped.is_empty());
    }
}
