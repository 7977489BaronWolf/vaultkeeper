#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::cli::import;
    use crate::store::Store;

    fn write_env_file(dir: &std::path::Path, content: &str) -> std::path::PathBuf {
        let path = dir.join(".env");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_import_basic_env_file() {
        let dir = tempdir().unwrap();
        let env_path = write_env_file(dir.path(), "FOO=bar\nBAZ=qux\n");
        let vault = "test_import_basic";
        Store::create(vault).unwrap();
        import::run(&env_path, Some(vault), false).unwrap();
        let store = Store::load(vault).unwrap();
        assert_eq!(store.get("FOO"), Some("bar"));
        assert_eq!(store.get("BAZ"), Some("qux"));
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_import_skips_comments_and_blanks() {
        let dir = tempdir().unwrap();
        let env_path = write_env_file(dir.path(), "# comment\n\nKEY=value\n");
        let vault = "test_import_comments";
        Store::create(vault).unwrap();
        import::run(&env_path, Some(vault), false).unwrap();
        let store = Store::load(vault).unwrap();
        assert_eq!(store.get("KEY"), Some("value"));
        assert_eq!(store.all().len(), 1);
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_import_force_overwrites() {
        let dir = tempdir().unwrap();
        let env_path = write_env_file(dir.path(), "KEY=newvalue\n");
        let vault = "test_import_force";
        Store::create(vault).unwrap();
        let mut store = Store::load(vault).unwrap();
        store.set("KEY", "oldvalue");
        store.save(vault).unwrap();
        import::run(&env_path, Some(vault), true).unwrap();
        let store = Store::load(vault).unwrap();
        assert_eq!(store.get("KEY"), Some("newvalue"));
        Store::delete(vault).unwrap();
    }

    #[test]
    fn test_import_strips_quotes() {
        let dir = tempdir().unwrap();
        let env_path = write_env_file(dir.path(), "KEY=\"quoted value\"\n");
        let vault = "test_import_quotes";
        Store::create(vault).unwrap();
        import::run(&env_path, Some(vault), false).unwrap();
        let store = Store::load(vault).unwrap();
        assert_eq!(store.get("KEY"), Some("quoted value"));
        Store::delete(vault).unwrap();
    }
}
