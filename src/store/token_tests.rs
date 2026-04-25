#[cfg(test)]
mod tests {
    use super::super::token::*;

    fn make_token(id: &str, scopes: Vec<&str>, expires_at: Option<u64>) -> Token {
        Token::new(id, &format!("label-{}", id), scopes.iter().map(|s| s.to_string()).collect(), expires_at)
    }

    #[test]
    fn test_token_not_expired() {
        let t = make_token("tok1", vec!["read"], None);
        assert!(!t.is_expired());
    }

    #[test]
    fn test_token_expired() {
        let t = make_token("tok2", vec!["read"], Some(1));
        assert!(t.is_expired());
    }

    #[test]
    fn test_has_scope_exact() {
        let t = make_token("tok3", vec!["read", "write"], None);
        assert!(t.has_scope("read"));
        assert!(t.has_scope("write"));
        assert!(!t.has_scope("delete"));
    }

    #[test]
    fn test_has_scope_wildcard() {
        let t = make_token("tok4", vec!["*"], None);
        assert!(t.has_scope("read"));
        assert!(t.has_scope("admin"));
    }

    #[test]
    fn test_store_add_and_get() {
        let mut store = TokenStore::default();
        let t = make_token("tok5", vec!["read"], None);
        store.add(t.clone());
        let fetched = store.get("tok5").unwrap();
        assert_eq!(fetched.label, "label-tok5");
    }

    #[test]
    fn test_store_remove() {
        let mut store = TokenStore::default();
        store.add(make_token("tok6", vec!["read"], None));
        let removed = store.remove("tok6");
        assert!(removed.is_some());
        assert!(store.get("tok6").is_none());
    }

    #[test]
    fn test_purge_expired() {
        let mut store = TokenStore::default();
        store.add(make_token("active", vec!["read"], None));
        store.add(make_token("dead", vec!["read"], Some(1)));
        let purged = store.purge_expired();
        assert_eq!(purged, 1);
        assert!(store.get("active").is_some());
        assert!(store.get("dead").is_none());
    }

    #[test]
    fn test_list_valid() {
        let mut store = TokenStore::default();
        store.add(make_token("v1", vec!["read"], None));
        store.add(make_token("v2", vec!["write"], None));
        store.add(make_token("exp", vec!["read"], Some(1)));
        let valid = store.list_valid();
        assert_eq!(valid.len(), 2);
    }
}
