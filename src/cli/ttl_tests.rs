#[cfg(test)]
mod tests {
    use super::super::ttl::{
        handle_ttl_check, handle_ttl_list, handle_ttl_purge, handle_ttl_set, load_ttl_store,
    };
    use crate::config::Config;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn test_config(dir: &std::path::Path) -> Config {
        Config {
            vault_dir: dir.to_path_buf(),
            public_key: "age1test".to_string(),
            default_env: "default".to_string(),
        }
    }

    #[test]
    fn test_set_and_load_ttl() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_set(&config, "DB_PASS", 120).unwrap();
        let store = load_ttl_store(&config).unwrap();
        assert!(store.entries.contains_key("DB_PASS"));
    }

    #[test]
    fn test_check_no_ttl() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_check(&config, "MISSING_KEY").unwrap();
    }

    #[test]
    fn test_check_with_ttl() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_set(&config, "API_KEY", 300).unwrap();
        handle_ttl_check(&config, "API_KEY").unwrap();
    }

    #[test]
    fn test_purge_expired() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_set(&config, "EXPIRED", 0).unwrap();
        handle_ttl_set(&config, "LIVE", 999).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        handle_ttl_purge(&config).unwrap();
        let store = load_ttl_store(&config).unwrap();
        assert!(!store.entries.contains_key("EXPIRED"));
        assert!(store.entries.contains_key("LIVE"));
    }

    #[test]
    fn test_list_empty() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_list(&config).unwrap();
    }

    #[test]
    fn test_list_with_entries() {
        let dir = tempdir().unwrap();
        let config = test_config(dir.path());
        handle_ttl_set(&config, "KEY1", 60).unwrap();
        handle_ttl_set(&config, "KEY2", 120).unwrap();
        handle_ttl_list(&config).unwrap();
    }
}
