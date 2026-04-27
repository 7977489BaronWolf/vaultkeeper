#[cfg(test)]
mod tests {
    use super::super::signing::*;

    #[test]
    fn test_sign_creates_entry() {
        let mut store = SigningStore::new();
        let entry = store.sign("MY_KEY", "my_value", "alice");
        assert_eq!(entry.key, "MY_KEY");
        assert_eq!(entry.signer, "alice");
        assert!(!entry.signature.is_empty());
    }

    #[test]
    fn test_verify_valid_signature() {
        let mut store = SigningStore::new();
        store.sign("MY_KEY", "my_value", "alice");
        assert!(store.verify("MY_KEY", "my_value"));
    }

    #[test]
    fn test_verify_tampered_value_fails() {
        let mut store = SigningStore::new();
        store.sign("MY_KEY", "original", "alice");
        assert!(!store.verify("MY_KEY", "tampered"));
    }

    #[test]
    fn test_verify_missing_key_returns_false() {
        let store = SigningStore::new();
        assert!(!store.verify("MISSING", "value"));
    }

    #[test]
    fn test_get_signature_returns_entry() {
        let mut store = SigningStore::new();
        store.sign("API_KEY", "secret123", "bob");
        let entry = store.get_signature("API_KEY");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().signer, "bob");
    }

    #[test]
    fn test_remove_entry() {
        let mut store = SigningStore::new();
        store.sign("KEY", "val", "alice");
        assert!(store.remove("KEY"));
        assert!(!store.verify("KEY", "val"));
    }

    #[test]
    fn test_remove_nonexistent_returns_false() {
        let mut store = SigningStore::new();
        assert!(!store.remove("GHOST"));
    }

    #[test]
    fn test_list_signed() {
        let mut store = SigningStore::new();
        store.sign("K1", "v1", "alice");
        store.sign("K2", "v2", "bob");
        assert_eq!(store.list_signed().len(), 2);
    }

    #[test]
    fn test_sign_overwrites_existing() {
        let mut store = SigningStore::new();
        store.sign("KEY", "old", "alice");
        store.sign("KEY", "new", "alice");
        assert!(!store.verify("KEY", "old"));
        assert!(store.verify("KEY", "new"));
    }
}
