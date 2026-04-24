#[cfg(test)]
mod tests {
    use super::super::hook::*;
    use crate::config::Config;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn test_config(dir: &TempDir) -> Config {
        Config {
            vault_dir: dir.path().to_path_buf(),
            default_env: "default".to_string(),
            key_path: dir.path().join("key.age"),
        }
    }

    #[test]
    fn test_add_and_list_hook() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        handle_hook_add(&config, "post-set", "echo set").unwrap();
        // list should not error
        handle_hook_list(&config).unwrap();
    }

    #[test]
    fn test_add_multiple_hooks() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        handle_hook_add(&config, "post-set", "echo a").unwrap();
        handle_hook_add(&config, "post-delete", "echo b").unwrap();
        let store_path = config.vault_dir.join(".hooks.json");
        let data = std::fs::read_to_string(store_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(parsed["hooks"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_remove_hook() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        handle_hook_add(&config, "post-rotate", "echo rotate").unwrap();
        handle_hook_remove(&config, "post-rotate", "echo rotate").unwrap();
        let store_path = config.vault_dir.join(".hooks.json");
        let data = std::fs::read_to_string(store_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(parsed["hooks"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_hook_no_error() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let result = handle_hook_remove(&config, "post-set", "echo missing");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_event_returns_error() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let result = handle_hook_add(&config, "on-magic", "echo nope");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_empty_hooks() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let result = handle_hook_list(&config);
        assert!(result.is_ok());
    }
}
