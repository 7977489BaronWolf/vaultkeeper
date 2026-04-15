#[cfg(test)]
mod tests {
    use super::super::snapshot::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn sample_entries() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("API_KEY".to_string(), "secret123".to_string());
        map.insert("DB_URL".to_string(), "postgres://localhost/db".to_string());
        map
    }

    #[test]
    fn test_snapshot_creation() {
        let entries = sample_entries();
        let snap = Snapshot::new("production", entries.clone(), Some("before-deploy".to_string()));
        assert_eq!(snap.env, "production");
        assert_eq!(snap.label, Some("before-deploy".to_string()));
        assert_eq!(snap.entries.len(), 2);
        assert!(!snap.id.is_empty());
    }

    #[test]
    fn test_save_and_load_snapshot() {
        let dir = tempdir().unwrap();
        let entries = sample_entries();
        let snap = Snapshot::new("staging", entries, None);
        save_snapshot(dir.path(), &snap).unwrap();
        let loaded = load_snapshots(dir.path(), "staging").unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, snap.id);
        assert_eq!(loaded[0].env, "staging");
    }

    #[test]
    fn test_load_filters_by_env() {
        let dir = tempdir().unwrap();
        let snap1 = Snapshot::new("staging", sample_entries(), None);
        let snap2 = Snapshot::new("production", sample_entries(), None);
        save_snapshot(dir.path(), &snap1).unwrap();
        save_snapshot(dir.path(), &snap2).unwrap();
        let staging = load_snapshots(dir.path(), "staging").unwrap();
        assert_eq!(staging.len(), 1);
        assert_eq!(staging[0].env, "staging");
    }

    #[test]
    fn test_delete_snapshot() {
        let dir = tempdir().unwrap();
        let snap = Snapshot::new("dev", sample_entries(), None);
        save_snapshot(dir.path(), &snap).unwrap();
        assert_eq!(load_snapshots(dir.path(), "dev").unwrap().len(), 1);
        delete_snapshot(dir.path(), &snap.id).unwrap();
        assert_eq!(load_snapshots(dir.path(), "dev").unwrap().len(), 0);
    }

    #[test]
    fn test_load_empty_dir() {
        let dir = tempdir().unwrap();
        let result = load_snapshots(dir.path(), "dev").unwrap();
        assert!(result.is_empty());
    }
}
