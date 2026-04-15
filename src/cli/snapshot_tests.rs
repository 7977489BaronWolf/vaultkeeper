#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tempfile::tempdir;
    use crate::config::Config;
    use crate::store::snapshot::{load_snapshots, save_snapshot, Snapshot};
    use crate::store::mod_plain::write_plain;

    fn make_config(dir: &std::path::Path) -> Config {
        Config {
            vault_dir: dir.to_path_buf(),
            default_env: "development".to_string(),
        }
    }

    fn sample_entries() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("KEY".to_string(), "value".to_string());
        m
    }

    #[test]
    fn test_snapshot_save_creates_file() {
        let dir = tempdir().unwrap();
        let config = make_config(dir.path());
        let plain_path = config.plain_path("development");
        std::fs::create_dir_all(plain_path.parent().unwrap()).unwrap();
        write_plain(&plain_path, &sample_entries()).unwrap();
        super::super::snapshot::handle_snapshot_save("development", Some("test-label".to_string()), &config).unwrap();
        let snaps = load_snapshots(&config.snapshots_dir(), "development").unwrap();
        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].label, Some("test-label".to_string()));
    }

    #[test]
    fn test_snapshot_list_empty() {
        let dir = tempdir().unwrap();
        let config = make_config(dir.path());
        let result = super::super::snapshot::handle_snapshot_list("development", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_restore() {
        let dir = tempdir().unwrap();
        let config = make_config(dir.path());
        let plain_path = config.plain_path("staging");
        std::fs::create_dir_all(plain_path.parent().unwrap()).unwrap();
        write_plain(&plain_path, &sample_entries()).unwrap();
        super::super::snapshot::handle_snapshot_save("staging", None, &config).unwrap();
        let mut updated = sample_entries();
        updated.insert("NEW_KEY".to_string(), "new_val".to_string());
        write_plain(&plain_path, &updated).unwrap();
        let snaps = load_snapshots(&config.snapshots_dir(), "staging").unwrap();
        super::super::snapshot::handle_snapshot_restore("staging", &snaps[0].id, &config).unwrap();
        let restored = crate::store::mod_plain::read_plain(&plain_path).unwrap();
        assert!(!restored.contains_key("NEW_KEY"));
    }

    #[test]
    fn test_snapshot_delete() {
        let dir = tempdir().unwrap();
        let config = make_config(dir.path());
        let snap = Snapshot::new("dev", sample_entries(), None);
        save_snapshot(&config.snapshots_dir(), &snap).unwrap();
        super::super::snapshot::handle_snapshot_delete("dev", &snap.id, &config).unwrap();
        let remaining = load_snapshots(&config.snapshots_dir(), "dev").unwrap();
        assert!(remaining.is_empty());
    }
}
