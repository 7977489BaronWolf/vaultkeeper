#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    use crate::config::Config;
    use crate::store::SecretStore;

    fn make_config(dir: &TempDir) -> Config {
        Config {
            vault_path: dir.path().join("vault.age").to_string_lossy().to_string(),
            key_path: dir.path().join("key.txt").to_string_lossy().to_string(),
            env_file: dir.path().join(".env").to_string_lossy().to_string(),
        }
    }

    #[test]
    fn test_status_no_vault_no_key() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        // Neither vault nor key exists — should still run without error
        let result = crate::cli::status::run(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_with_vault_only() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        // Create a dummy vault file
        fs::write(&config.vault_path, b"dummy").unwrap();

        let result = crate::cli::status::run(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_with_key_only() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        fs::write(&config.key_path, b"AGE-SECRET-KEY-FAKE").unwrap();

        let result = crate::cli::status::run(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_with_populated_store() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let mut store = SecretStore::new();
        store.set("API_KEY".to_string(), "secret123".to_string());
        store.set("DB_PASS".to_string(), "hunter2".to_string());
        store.save(std::path::Path::new(&config.vault_path)).unwrap();

        fs::write(&config.key_path, b"AGE-SECRET-KEY-FAKE").unwrap();

        let result = crate::cli::status::run(&config);
        assert!(result.is_ok());
    }
}
