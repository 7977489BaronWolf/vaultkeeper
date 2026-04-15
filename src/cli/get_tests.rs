#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::cli::get::run_get;
    use crate::store::VaultStore;
    use crate::config::Config;

    fn setup_vault(dir: &std::path::Path) -> (Config, String) {
        let key_path = dir.join("test.key");
        let vault_path = dir.join(".vault");

        // Generate a key pair
        let identity = age::x25519::Identity::generate();
        let pubkey = identity.to_public().to_string();
        let privkey = identity.to_string().expose_secret().to_string();
        fs::write(&key_path, &privkey).unwrap();

        let config = Config {
            vault_path: vault_path.to_string_lossy().to_string(),
            public_key: pubkey.clone(),
            key_path: key_path.to_string_lossy().to_string(),
        };

        (config, pubkey)
    }

    #[test]
    fn test_get_existing_key() {
        let dir = tempdir().unwrap();
        let env_file = dir.path().join(".env");
        fs::write(&env_file, "API_KEY=secret123\nDEBUG=true\n").unwrap();

        let (config, _) = setup_vault(dir.path());
        let mut store = VaultStore::new(&config.vault_path);
        store.set("API_KEY", "secret123");
        store.set("DEBUG", "true");

        let result = run_get(&store, "API_KEY");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "secret123");
    }

    #[test]
    fn test_get_missing_key_returns_error() {
        let dir = tempdir().unwrap();
        let (config, _) = setup_vault(dir.path());
        let store = VaultStore::new(&config.vault_path);

        let result = run_get(&store, "NONEXISTENT_KEY");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NONEXISTENT_KEY"));
    }

    #[test]
    fn test_get_empty_store_returns_error() {
        let dir = tempdir().unwrap();
        let (config, _) = setup_vault(dir.path());
        let store = VaultStore::new(&config.vault_path);

        let result = run_get(&store, "ANY_KEY");
        assert!(result.is_err());
    }
}
