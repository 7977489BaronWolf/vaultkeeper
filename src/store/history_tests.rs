#[cfg(test)]
mod tests {
    use super::super::history::History;
    use tempfile::tempdir;

    #[test]
    fn test_push_and_retrieve() {
        let mut h = History::default();
        h.push("set", "API_KEY", "dev");
        h.push("delete", "DB_PASS", "prod");
        assert_eq!(h.entries.len(), 2);
    }

    #[test]
    fn test_filter_by_env() {
        let mut h = History::default();
        h.push("set", "KEY1", "dev");
        h.push("set", "KEY2", "prod");
        h.push("delete", "KEY3", "dev");
        let dev: Vec<_> = h.for_env("dev").collect();
        assert_eq!(dev.len(), 2);
        assert!(dev.iter().all(|e| e.env == "dev"));
    }

    #[test]
    fn test_max_history_cap() {
        let mut h = History::default();
        for i in 0..55 {
            h.push("set", &format!("KEY_{}", i), "dev");
        }
        assert_eq!(h.entries.len(), 50);
        assert_eq!(h.entries.front().unwrap().key, "KEY_5");
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("history.json");
        let mut h = History::default();
        h.push("set", "TOKEN", "staging");
        h.save(&path).unwrap();
        let loaded = History::load(&path).unwrap();
        assert_eq!(loaded.entries.len(), 1);
        assert_eq!(loaded.entries[0].key, "TOKEN");
    }

    #[test]
    fn test_clear() {
        let mut h = History::default();
        h.push("set", "X", "dev");
        h.clear();
        assert!(h.entries.is_empty());
    }

    #[test]
    fn test_load_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let h = History::load(&path).unwrap();
        assert!(h.entries.is_empty());
    }
}
