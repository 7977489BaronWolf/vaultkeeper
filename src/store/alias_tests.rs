#[cfg(test)]
mod tests {
    use crate::store::alias::AliasStore;

    fn make_store() -> AliasStore {
        let mut s = AliasStore::new();
        s.set("DB_URL", "DATABASE_URL").unwrap();
        s.set("PG", "DATABASE_URL").unwrap();
        s
    }

    #[test]
    fn test_set_and_get() {
        let s = make_store();
        assert_eq!(s.get("DB_URL"), Some(&"DATABASE_URL".to_string()));
    }

    #[test]
    fn test_get_missing_returns_none() {
        let s = make_store();
        assert!(s.get("UNKNOWN").is_none());
    }

    #[test]
    fn test_resolve_alias() {
        let s = make_store();
        assert_eq!(s.resolve("DB_URL"), "DATABASE_URL");
    }

    #[test]
    fn test_resolve_non_alias_returns_key() {
        let s = make_store();
        assert_eq!(s.resolve("SOME_KEY"), "SOME_KEY");
    }

    #[test]
    fn test_remove_existing() {
        let mut s = make_store();
        assert!(s.remove("DB_URL").is_ok());
        assert!(s.get("DB_URL").is_none());
    }

    #[test]
    fn test_remove_missing_returns_err() {
        let mut s = make_store();
        assert!(s.remove("NONEXISTENT").is_err());
    }

    #[test]
    fn test_set_empty_alias_errors() {
        let mut s = AliasStore::new();
        assert!(s.set("", "TARGET").is_err());
    }

    #[test]
    fn test_set_self_reference_errors() {
        let mut s = AliasStore::new();
        assert!(s.set("KEY", "KEY").is_err());
    }

    #[test]
    fn test_list_sorted() {
        let s = make_store();
        let list = s.list();
        assert_eq!(list[0].0, "DB_URL");
        assert_eq!(list[1].0, "PG");
    }

    #[test]
    fn test_cycle_detection() {
        let mut s = AliasStore::new();
        s.set("A", "B").unwrap();
        s.set("B", "C").unwrap();
        assert!(s.has_cycle("C", "A"));
        assert!(!s.has_cycle("D", "A"));
    }
}
