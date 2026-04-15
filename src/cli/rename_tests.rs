#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::store::Store;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn setup_store_with_env(store_path: &PathBuf, env_name: &str) {
        let mut store = Store::new();
        store.add_env(env_name);
        store.set(env_name, "KEY", "value").unwrap();
        store.save(store_path).unwrap();
    }

    fn make_config(store_path: PathBuf) -> Config {
        Config {
            store_path,
            default_env: "default".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_rename_success() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        setup_store_with_env(&store_path, "staging");

        let config = make_config(store_path.clone());
        let config_file = dir.path().join("vaultkeeper.toml");
        config.save(&config_file).unwrap();

        let result = crate::cli::rename::run("staging", "production", Some(config_file));
        assert!(result.is_ok(), "{:?}", result);

        let store = Store::load(&store_path).unwrap();
        assert!(!store.has_env("staging"));
        assert!(store.has_env("production"));
    }

    #[test]
    fn test_rename_same_name_fails() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        setup_store_with_env(&store_path, "staging");

        let config = make_config(store_path.clone());
        let config_file = dir.path().join("vaultkeeper.toml");
        config.save(&config_file).unwrap();

        let result = crate::cli::rename::run("staging", "staging", Some(config_file));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("same"));
    }

    #[test]
    fn test_rename_nonexistent_env_fails() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        let store = Store::new();
        store.save(&store_path).unwrap();

        let config = make_config(store_path.clone());
        let config_file = dir.path().join("vaultkeeper.toml");
        config.save(&config_file).unwrap();

        let result = crate::cli::rename::run("ghost", "phantom", Some(config_file));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_rename_to_existing_env_fails() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        let mut store = Store::new();
        store.add_env("alpha");
        store.add_env("beta");
        store.save(&store_path).unwrap();

        let config = make_config(store_path.clone());
        let config_file = dir.path().join("vaultkeeper.toml");
        config.save(&config_file).unwrap();

        let result = crate::cli::rename::run("alpha", "beta", Some(config_file));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
