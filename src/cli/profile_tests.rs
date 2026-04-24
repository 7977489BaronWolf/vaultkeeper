#[cfg(test)]
mod tests {
    use super::super::profile::*;
    use tempfile::tempdir;

    fn profile_path(dir: &tempfile::TempDir) -> std::path::PathBuf {
        dir.path().join("profiles.json")
    }

    #[test]
    fn test_add_and_list_profile() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "dev", Some("Development env")).unwrap();
        handle_profile_add(&path, "prod", Some("Production env")).unwrap();
        // list should not error
        assert!(handle_profile_list(&path).is_ok());
    }

    #[test]
    fn test_add_duplicate_fails() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "dev", None).unwrap();
        let err = handle_profile_add(&path, "dev", None).unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_remove_profile() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "staging", None).unwrap();
        assert!(handle_profile_remove(&path, "staging").is_ok());
        let err = handle_profile_remove(&path, "staging").unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_activate_profile() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "dev", None).unwrap();
        assert!(handle_profile_activate(&path, "dev").is_ok());
    }

    #[test]
    fn test_activate_missing_profile_fails() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        let err = handle_profile_activate(&path, "ghost").unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_deactivate_profile() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "dev", None).unwrap();
        handle_profile_activate(&path, "dev").unwrap();
        assert!(handle_profile_deactivate(&path).is_ok());
    }

    #[test]
    fn test_show_profile() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        handle_profile_add(&path, "dev", Some("Dev profile")).unwrap();
        assert!(handle_profile_show(&path, "dev").is_ok());
    }

    #[test]
    fn test_show_missing_profile_fails() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        let err = handle_profile_show(&path, "nope").unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_list_empty_store() {
        let dir = tempdir().unwrap();
        let path = profile_path(&dir);
        assert!(handle_profile_list(&path).is_ok());
    }
}
