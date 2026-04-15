#[cfg(test)]
mod tests {
    use crate::cli::unset;
    use crate::config::Config;
    use crate::store::Store;
    use std::fs;
    use tempfile::tempdir;

    fn setup_vault(dir: &std::path::Path) -> Config {
        let config = Config::new_for_test(dir);
        let mut store = Store::new();
        store.set("DB_HOST", "localhost");
        store.set("DB_PORT", "5432");
        store.set("API_KEY", "supersecret");
        store
            .save_encrypted(&config.vault_path(), &config.recipient_path())
            .expect("save failed");
        config
    }

    #[test]
    fn removes_existing_key() {
        let dir = tempdir().unwrap();
        let config = setup_vault(dir.path());

        unset::run(&["DB_HOST".to_string()], &config).expect("unset failed");

        let store = Store::load_encrypted(&config.vault_path(), &config.identity_path())
            .expect("reload failed");
        assert!(store.get("DB_HOST").is_none());
        assert!(store.get("DB_PORT").is_some());
    }

    #[test]
    fn removes_multiple_keys() {
        let dir = tempdir().unwrap();
        let config = setup_vault(dir.path());

        unset::run(
            &["DB_HOST".to_string(), "API_KEY".to_string()],
            &config,
        )
        .expect("unset failed");

        let store = Store::load_encrypted(&config.vault_path(), &config.identity_path())
            .expect("reload failed");
        assert!(store.get("DB_HOST").is_none());
        assert!(store.get("API_KEY").is_none());
        assert!(store.get("DB_PORT").is_some());
    }

    #[test]
    fn errors_when_no_keys_provided() {
        let dir = tempdir().unwrap();
        let config = setup_vault(dir.path());
        let result = unset::run(&[], &config);
        assert!(result.is_err());
    }

    #[test]
    fn errors_when_all_keys_missing() {
        let dir = tempdir().unwrap();
        let config = setup_vault(dir.path());
        let result = unset::run(&["NONEXISTENT".to_string()], &config);
        assert!(result.is_err());
    }

    #[test]
    fn partial_removal_succeeds_with_warning() {
        let dir = tempdir().unwrap();
        let config = setup_vault(dir.path());

        // DB_PORT exists, GHOST does not — should succeed (partial)
        unset::run(
            &["DB_PORT".to_string(), "GHOST".to_string()],
            &config,
        )
        .expect("partial unset should succeed");

        let store = Store::load_encrypted(&config.vault_path(), &config.identity_path())
            .expect("reload failed");
        assert!(store.get("DB_PORT").is_none());
        assert!(store.get("DB_HOST").is_some());
    }
}
