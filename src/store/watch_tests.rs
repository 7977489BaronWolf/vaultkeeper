#[cfg(test)]
mod tests {
    use super::super::watch::WatchIndex;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_temp_file(content: &[u8]) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn test_track_and_is_not_modified() {
        let file = make_temp_file(b"hello");
        let mut idx = WatchIndex::new();
        idx.track("dev", file.path()).unwrap();
        // immediately after tracking, should not be modified
        assert!(!idx.is_modified("dev", file.path()).unwrap());
    }

    #[test]
    fn test_untracked_is_modified() {
        let file = make_temp_file(b"hello");
        let idx = WatchIndex::new();
        assert!(idx.is_modified("dev", file.path()).unwrap());
    }

    #[test]
    fn test_untrack_removes_entry() {
        let file = make_temp_file(b"hello");
        let mut idx = WatchIndex::new();
        idx.track("dev", file.path()).unwrap();
        idx.untrack("dev");
        assert!(idx.is_modified("dev", file.path()).unwrap());
    }

    #[test]
    fn test_tracked_names() {
        let f1 = make_temp_file(b"a");
        let f2 = make_temp_file(b"b");
        let mut idx = WatchIndex::new();
        idx.track("dev", f1.path()).unwrap();
        idx.track("prod", f2.path()).unwrap();
        let mut names: Vec<&String> = idx.tracked_names();
        names.sort();
        assert_eq!(names, vec!["dev", "prod"]);
    }

    #[test]
    fn test_size_change_detected() {
        let mut file = make_temp_file(b"hi");
        let mut idx = WatchIndex::new();
        idx.track("dev", file.path()).unwrap();
        // overwrite with different size
        file.write_all(b"hello world extended").unwrap();
        file.flush().unwrap();
        assert!(idx.is_modified("dev", file.path()).unwrap());
    }
}
