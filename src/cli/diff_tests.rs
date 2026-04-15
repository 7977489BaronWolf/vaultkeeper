#[cfg(test)]
mod tests {
    use crate::cli::diff::run_diff;
    use crate::config::Config;
    use std::fs;
    use tempfile::TempDir;

    fn make_config(dir: &TempDir) -> Config {
        Config::new(
            dir.path().join(".env").to_str().unwrap().to_string(),
            dir.path().join(".env.age").to_str().unwrap().to_string(),
            dir.path().join("key.txt").to_str().unwrap().to_string(),
        )
    }

    #[test]
    fn test_diff_identical_files() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let content = "FOO=bar\nBAZ=qux\n";
        let file_a = dir.path().join("a.env");
        let file_b = dir.path().join("b.env");
        fs::write(&file_a, content).unwrap();
        fs::write(&file_b, content).unwrap();

        let result = run_diff(
            &config,
            file_a.to_str().unwrap(),
            Some(file_b.to_str().unwrap()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_missing_key_in_b() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let file_a = dir.path().join("a.env");
        let file_b = dir.path().join("b.env");
        fs::write(&file_a, "FOO=bar\nBAZ=qux\n").unwrap();
        fs::write(&file_b, "FOO=bar\n").unwrap();

        let result = run_diff(
            &config,
            file_a.to_str().unwrap(),
            Some(file_b.to_str().unwrap()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_changed_value() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let file_a = dir.path().join("a.env");
        let file_b = dir.path().join("b.env");
        fs::write(&file_a, "FOO=old\n").unwrap();
        fs::write(&file_b, "FOO=new\n").unwrap();

        let result = run_diff(
            &config,
            file_a.to_str().unwrap(),
            Some(file_b.to_str().unwrap()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_nonexistent_file_errors() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let result = run_diff(&config, "/nonexistent/path.env", None);
        assert!(result.is_err());
    }
}
