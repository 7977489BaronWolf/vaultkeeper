#[cfg(test)]
mod tests {
    use crate::store::remote::{RemoteConfig, RemoteStore};

    fn make_remote(name: &str, url: &str) -> RemoteConfig {
        RemoteConfig {
            name: name.to_string(),
            url: url.to_string(),
            auth_token: None,
        }
    }

    #[test]
    fn test_add_and_get_remote() {
        let mut store = RemoteStore::new();
        let cfg = make_remote("origin", "https://vault.example.com");
        store.add(cfg.clone()).unwrap();
        let got = store.get("origin").unwrap();
        assert_eq!(got.url, "https://vault.example.com");
    }

    #[test]
    fn test_add_duplicate_returns_error() {
        let mut store = RemoteStore::new();
        store.add(make_remote("origin", "https://a.example.com")).unwrap();
        let err = store.add(make_remote("origin", "https://b.example.com")).unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_remove_remote() {
        let mut store = RemoteStore::new();
        store.add(make_remote("backup", "https://backup.example.com")).unwrap();
        let removed = store.remove("backup").unwrap();
        assert_eq!(removed.name, "backup");
        assert!(store.get("backup").is_none());
    }

    #[test]
    fn test_remove_nonexistent_returns_error() {
        let mut store = RemoteStore::new();
        let err = store.remove("ghost").unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_list_sorted() {
        let mut store = RemoteStore::new();
        store.add(make_remote("zeta", "https://z.example.com")).unwrap();
        store.add(make_remote("alpha", "https://a.example.com")).unwrap();
        let list = store.list();
        assert_eq!(list[0].name, "alpha");
        assert_eq!(list[1].name, "zeta");
    }

    #[test]
    fn test_update_token() {
        let mut store = RemoteStore::new();
        store.add(make_remote("origin", "https://vault.example.com")).unwrap();
        store.update_token("origin", Some("tok_abc123".to_string())).unwrap();
        assert_eq!(store.get("origin").unwrap().auth_token.as_deref(), Some("tok_abc123"));
    }

    #[test]
    fn test_update_token_nonexistent_returns_error() {
        let mut store = RemoteStore::new();
        let err = store.update_token("ghost", Some("tok".to_string())).unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_count_and_is_empty() {
        let mut store = RemoteStore::new();
        assert!(store.is_empty());
        store.add(make_remote("r1", "https://r1.example.com")).unwrap();
        assert!(!store.is_empty());
        assert_eq!(store.count(), 1);
    }
}
