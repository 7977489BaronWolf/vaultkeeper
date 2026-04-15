#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    use crate::cli::copy::run_copy;
    use crate::cli::init::run_init;
    use crate::cli::set::run_set;

    fn setup_vault(dir: &TempDir) {
        run_init(Some("default"), &dir.path().to_path_buf())
            .expect("init failed");
        run_set("API_KEY", "supersecret", None, &dir.path().to_path_buf())
            .expect("set failed");
    }

    #[test]
    fn test_copy_key_succeeds() {
        let dir = TempDir::new().unwrap();
        setup_vault(&dir);

        let result = run_copy(
            "API_KEY",
            "API_KEY_BACKUP",
            None,
            &dir.path().to_path_buf(),
        );
        assert!(result.is_ok(), "copy should succeed: {:?}", result);
    }

    #[test]
    fn test_copy_missing_source_key_fails() {
        let dir = TempDir::new().unwrap();
        setup_vault(&dir);

        let result = run_copy(
            "NONEXISTENT",
            "DEST_KEY",
            None,
            &dir.path().to_path_buf(),
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not found"));
    }

    #[test]
    fn test_copy_to_existing_key_fails() {
        let dir = TempDir::new().unwrap();
        setup_vault(&dir);
        run_set("API_KEY_BACKUP", "otherval", None, &dir.path().to_path_buf())
            .expect("set failed");

        let result = run_copy(
            "API_KEY",
            "API_KEY_BACKUP",
            None,
            &dir.path().to_path_buf(),
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("already exists"));
    }

    #[test]
    fn test_copy_vault_does_not_exist_fails() {
        let dir = TempDir::new().unwrap();
        // no init
        let _ = fs::create_dir_all(dir.path());

        let result = run_copy(
            "API_KEY",
            "API_KEY_BACKUP",
            None,
            &dir.path().to_path_buf(),
        );
        assert!(result.is_err());
    }
}
