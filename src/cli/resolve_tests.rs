#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::store::Store;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn make_config_with_secrets(pairs: &[(&str, &str)]) -> (Config, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let vault_path = dir.path().join("vault.json");
        let mut store = Store::new();
        for (k, v) in pairs {
            store.set_plain(k, v);
        }
        store.save(&vault_path).unwrap();
        let config = Config {
            vault_path,
            ..Default::default()
        };
        (config, dir)
    }

    #[test]
    fn test_resolve_key_plain() {
        let (config, _dir) = make_config_with_secrets(&[("FOO", "bar")]);
        let result = super::super::resolve::resolve_key(&config, "FOO");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "bar");
    }

    #[test]
    fn test_resolve_key_with_reference() {
        let (config, _dir) = make_config_with_secrets(&[
            ("HOST", "localhost"),
            ("DSN", "postgres://${HOST}/db"),
        ]);
        let result = super::super::resolve::resolve_key(&config, "DSN");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "postgres://localhost/db");
    }

    #[test]
    fn test_resolve_key_missing() {
        let (config, _dir) = make_config_with_secrets(&[("A", "1")]);
        let result = super::super::resolve::resolve_key(&config, "MISSING");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_key_cycle_error() {
        let (config, _dir) = make_config_with_secrets(&[
            ("A", "${B}"),
            ("B", "${A}"),
        ]);
        let result = super::super::resolve::resolve_key(&config, "A");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Circular") || msg.contains("circular"));
    }

    #[test]
    fn test_run_resolve_no_cycles() {
        let (config, _dir) = make_config_with_secrets(&[
            ("X", "hello"),
            ("Y", "${X} world"),
        ]);
        let result = super::super::resolve::run_resolve(&config, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_resolve_cycles_only_flag() {
        let (config, _dir) = make_config_with_secrets(&[("Z", "plain")]);
        let result = super::super::resolve::run_resolve(&config, true);
        assert!(result.is_ok());
    }
}
