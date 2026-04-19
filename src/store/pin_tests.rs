#[cfg(test)]
mod tests {
    use super::super::pin::PinStore;

    #[test]
    fn test_pin_and_is_pinned() {
        let mut store = PinStore::new();
        store.pin("SECRET_KEY");
        assert!(store.is_pinned("SECRET_KEY"));
        assert!(!store.is_pinned("OTHER_KEY"));
    }

    #[test]
    fn test_unpin() {
        let mut store = PinStore::new();
        store.pin("SECRET_KEY");
        store.unpin("SECRET_KEY");
        assert!(!store.is_pinned("SECRET_KEY"));
    }

    #[test]
    fn test_list_pinned() {
        let mut store = PinStore::new();
        store.pin("A");
        store.pin("B");
        let mut pinned = store.list_pinned();
        pinned.sort();
        assert_eq!(pinned, vec!["A", "B"]);
    }

    #[test]
    fn test_filter_unpinned() {
        let mut store = PinStore::new();
        store.pin("PINNED");
        let keys = vec!["PINNED", "FREE", "ALSO_FREE"];
        let result = store.filter_unpinned(&keys);
        assert!(!result.contains(&"PINNED"));
        assert!(result.contains(&"FREE"));
        assert!(result.contains(&"ALSO_FREE"));
    }

    #[test]
    fn test_unpin_nonexistent_is_noop() {
        let mut store = PinStore::new();
        store.unpin("GHOST");
        assert!(!store.is_pinned("GHOST"));
    }

    #[test]
    fn test_empty_store() {
        let store = PinStore::new();
        assert!(store.list_pinned().is_empty());
    }
}
