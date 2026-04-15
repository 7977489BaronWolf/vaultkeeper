#[cfg(test)]
mod tests {
    use crate::cli::clone;
    use crate::config::Config;
    use crate::store;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn make_config(vault_dir: &str) -> Config {
        Config {
            vault_dir: vault_dir.to_string(),
            public_key: "age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8zmrj2kg5sfn9aqmcac8p".to_string(),
            identity_file: "/tmp/test_key.txt".to_string(),
            default_env: "default".to_string(),
        }
    }

    #[test]
    fn test_clone_success() {
        let dir = tempdir().unwrap();
        let vault_dir = dir.path().to_str().unwrap();
        let config = make_config(vault_dir);

        let mut secrets = HashMap::new();
        secrets.insert("KEY".to_string(), "value".to_string());
        store::save_secrets_plain(vault_dir, "staging", &secrets).unwrap();

        let result = clone::run("staging", "production", &config);
        assert!(result.is_ok(), "{:?}", result);

        let loaded = store::load_secrets_plain(vault_dir, "production").unwrap();
        assert_eq!(loaded.get("KEY").map(|s| s.as_str()), Some("value"));
    }

    #[test]
    fn test_clone_source_missing() {
        let dir = tempdir().unwrap();
        let vault_dir = dir.path().to_str().unwrap();
        let config = make_config(vault_dir);

        let result = clone::run("nonexistent", "production", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_destination_exists() {
        let dir = tempdir().unwrap();
        let vault_dir = dir.path().to_str().unwrap();
        let config = make_config(vault_dir);

        let mut secrets = HashMap::new();
        secrets.insert("KEY".to_string(), "val".to_string());
        store::save_secrets_plain(vault_dir, "staging", &secrets).unwrap();
        store::save_secrets_plain(vault_dir, "production", &secrets).unwrap();

        let result = clone::run("staging", "production", &config);
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("already exists"));
    }
}
