#[cfg(test)]
mod tests {
    use super::super::profile::{Profile, ProfileStore};

    fn make_store() -> ProfileStore {
        let mut store = ProfileStore::new();
        store.add(Profile::new("dev").with_description("Development")).unwrap();
        store.add(Profile::new("prod").with_description("Production")).unwrap();
        store
    }

    #[test]
    fn test_add_profile() {
        let mut store = ProfileStore::new();
        assert!(store.add(Profile::new("staging")).is_ok());
        assert_eq!(store.profiles.len(), 1);
    }

    #[test]
    fn test_add_duplicate_profile_fails() {
        let mut store = make_store();
        let err = store.add(Profile::new("dev")).unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_remove_profile() {
        let mut store = make_store();
        assert!(store.remove("dev").is_ok());
        assert!(!store.profiles.contains_key("dev"));
    }

    #[test]
    fn test_remove_nonexistent_fails() {
        let mut store = make_store();
        assert!(store.remove("ghost").is_err());
    }

    #[test]
    fn test_cannot_remove_active_profile() {
        let mut store = make_store();
        store.activate("dev").unwrap();
        let err = store.remove("dev").unwrap_err();
        assert!(err.contains("active profile"));
    }

    #[test]
    fn test_activate_profile() {
        let mut store = make_store();
        assert!(store.activate("prod").is_ok());
        assert_eq!(store.active.as_deref(), Some("prod"));
    }

    #[test]
    fn test_activate_missing_profile_fails() {
        let mut store = make_store();
        assert!(store.activate("nope").is_err());
    }

    #[test]
    fn test_get_active_returns_profile() {
        let mut store = make_store();
        store.activate("dev").unwrap();
        let active = store.get_active().unwrap();
        assert_eq!(active.name, "dev");
    }

    #[test]
    fn test_deactivate_clears_active() {
        let mut store = make_store();
        store.activate("dev").unwrap();
        store.deactivate();
        assert!(store.active.is_none());
    }

    #[test]
    fn test_profile_overrides() {
        let mut p = Profile::new("test");
        p.set_override("DB_HOST", "localhost");
        assert_eq!(p.env_overrides.get("DB_HOST").unwrap(), "localhost");
        assert!(p.remove_override("DB_HOST"));
        assert!(!p.env_overrides.contains_key("DB_HOST"));
    }

    #[test]
    fn test_list_sorted() {
        let store = make_store();
        let names: Vec<&str> = store.list().iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["dev", "prod"]);
    }
}
