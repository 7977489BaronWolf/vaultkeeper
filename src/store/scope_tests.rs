#[cfg(test)]
mod tests {
    use super::super::scope::*;
    use std::collections::HashMap;

    fn make_scopes() -> HashMap<String, Scope> {
        let mut root = Scope::new("root", None);
        root.set("BASE_URL", "https://example.com");
        root.set("TIMEOUT", "30");

        let mut child = Scope::new("child", Some("root"));
        child.set("TIMEOUT", "60");
        child.set("CHILD_ONLY", "yes");

        let mut map = HashMap::new();
        map.insert("root".to_string(), root);
        map.insert("child".to_string(), child);
        map
    }

    #[test]
    fn test_scope_set_get() {
        let mut scope = Scope::new("test", None);
        scope.set("KEY", "value");
        assert_eq!(scope.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_scope_get_missing() {
        let scope = Scope::new("test", None);
        assert!(scope.get("MISSING").is_none());
    }

    #[test]
    fn test_scope_remove() {
        let mut scope = Scope::new("test", None);
        scope.set("KEY", "val");
        assert!(scope.remove("KEY"));
        assert!(scope.get("KEY").is_none());
    }

    #[test]
    fn test_scope_remove_missing() {
        let mut scope = Scope::new("test", None);
        assert!(!scope.remove("NOPE"));
    }

    #[test]
    fn test_resolve_local_override() {
        let scopes = make_scopes();
        let child = scopes.get("child").unwrap();
        let val = resolve_with_parent("TIMEOUT", child, &scopes);
        assert_eq!(val, Some(&"60".to_string()));
    }

    #[test]
    fn test_resolve_inherits_from_parent() {
        let scopes = make_scopes();
        let child = scopes.get("child").unwrap();
        let val = resolve_with_parent("BASE_URL", child, &scopes);
        assert_eq!(val, Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_resolve_not_found() {
        let scopes = make_scopes();
        let child = scopes.get("child").unwrap();
        let val = resolve_with_parent("UNKNOWN", child, &scopes);
        assert!(val.is_none());
    }

    #[test]
    fn test_list_all_scopes_sorted() {
        let scopes = make_scopes();
        let names = list_all_scopes(&scopes);
        assert_eq!(names, vec!["child", "root"]);
    }
}
