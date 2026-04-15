#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::cli::export;
    use crate::store::Store;

    #[test]
    fn test_export_creates_env_file() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("exported.env");
        let vault = "test_export_basic";
        Store::create(vault).unwrap();
        let mut store = Store::load(vault).unwrap();
        store.set("ALPHA", "one");
        store.set("BETA", "two");
        store.save(vault).unwrap();
        export::run(&output, Some(vault), false).unwrap();
        assert!(output.exists());
        let content = fs::read_to_string(&output).unwrap();
        assert!(content.contains("ALPHA=one"));
        assert!(content.contains("BETA=two"));
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_export_fails_if_file_exists_no_overwrite() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("existing.env");
        fs::write(&output, "existing content").unwrap();
        let vault = "test_export_no_overwrite";
        Store::create(vault).unwrap();
        let result = export::run(&output, Some(vault), false);
        assert!(result.is_err());
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_export_overwrites_with_flag() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("overwrite.env");
        fs::write(&output, "old content").unwrap();
        let vault = "test_export_overwrite";
        Store::create(vault).unwrap();
        let mut store = Store::load(vault).unwrap();
        store.set("NEW_KEY", "new_val");
        store.save(vault).unwrap();
        export::run(&output, Some(vault), true).unwrap();
        let content = fs::read_to_string(&output).unwrap();
        assert!(content.contains("NEW_KEY=new_val"));
        assert!(!content.contains("old content"));
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_export_quotes_values_with_spaces() {
        let dir = tempdir().unwrap();
        let output = dir.path().join("spaces.env");
        let vault = "test_export_spaces";
        Store::create(vault).unwrap();
        let mut store = Store::load(vault).unwrap();
        store.set("MSG", "hello world");
        store.save(vault).unwrap();
        export::run(&output, Some(vault), false).unwrap();
        let content = fs::read_to_string(&output).unwrap();
        assert!(content.contains("MSG=\"hello world\""));
        Store::delete(vault).unwrap();
    }
}
