#[cfg(test)]
mod tests {
    use super::super::signing::*;
    use crate::store::signing::SigningStore;
    use tempfile::NamedTempFile;

    #[test]
    fn test_handle_sign_adds_entry() {
        let mut store = SigningStore::new();
        handle_sign(&mut store, "DB_PASS", "s3cr3t", "alice");
        assert!(store.verify("DB_PASS", "s3cr3t"));
    }

    #[test]
    fn test_handle_verify_valid() {
        let mut store = SigningStore::new();
        handle_sign(&mut store, "API_KEY", "abc123", "bob");
        // should not panic or exit
        handle_verify(&store, "API_KEY", "abc123");
    }

    #[test]
    fn test_handle_show_signature_found() {
        let mut store = SigningStore::new();
        handle_sign(&mut store, "TOKEN", "xyz", "carol");
        // should not panic
        handle_show_signature(&store, "TOKEN");
    }

    #[test]
    fn test_handle_list_signed_empty() {
        let store = SigningStore::new();
        // should print "No signed entries." without panic
        handle_list_signed(&store);
    }

    #[test]
    fn test_handle_list_signed_multiple() {
        let mut store = SigningStore::new();
        handle_sign(&mut store, "Z_KEY", "v1", "alice");
        handle_sign(&mut store, "A_KEY", "v2", "bob");
        handle_list_signed(&store);
        assert_eq!(store.list_signed().len(), 2);
    }

    #[test]
    fn test_handle_unsign_existing() {
        let mut store = SigningStore::new();
        handle_sign(&mut store, "OLD_KEY", "val", "alice");
        handle_unsign(&mut store, "OLD_KEY");
        assert!(!store.verify("OLD_KEY", "val"));
    }

    #[test]
    fn test_save_and_load_store() {
        let tmp = NamedTempFile::new().unwrap();
        let mut store = SigningStore::new();
        store.sign("PERSIST_KEY", "persist_val", "dave");
        save_signing_store(&store, tmp.path()).unwrap();
        let loaded = load_signing_store(tmp.path());
        assert!(loaded.verify("PERSIST_KEY", "persist_val"));
    }

    #[test]
    fn test_load_nonexistent_returns_empty() {
        let path = std::path::Path::new("/tmp/vaultkeeper_signing_nonexistent_xyz.json");
        let store = load_signing_store(path);
        assert_eq!(store.list_signed().len(), 0);
    }
}
