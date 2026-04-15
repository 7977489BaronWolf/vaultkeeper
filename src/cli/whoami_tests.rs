use super::whoami::run;
use crate::config::Config;
use std::fs;
use tempfile::TempDir;

fn make_config_with_key(dir: &TempDir, public_key: Option<&str>) -> std::path::PathBuf {
    let cfg_path = dir.path().join("config.toml");
    let store_path = dir.path().join("store");
    fs::create_dir_all(&store_path).unwrap();

    let mut config = Config::new(store_path);
    if let Some(key) = public_key {
        config.set_public_key(key.to_string());
    }
    config.save(&cfg_path).unwrap();
    cfg_path
}

#[test]
fn test_whoami_with_key_succeeds() {
    let dir = TempDir::new().unwrap();
    let cfg_path = make_config_with_key(
        &dir,
        Some("age1qyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqsabcdef"),
    );
    let result = run(Some(cfg_path));
    assert!(result.is_ok(), "whoami should succeed with a configured key");
}

#[test]
fn test_whoami_without_key_succeeds() {
    let dir = TempDir::new().unwrap();
    let cfg_path = make_config_with_key(&dir, None);
    let result = run(Some(cfg_path));
    assert!(
        result.is_ok(),
        "whoami should succeed even without a public key"
    );
}

#[test]
fn test_whoami_missing_config_fails() {
    let dir = TempDir::new().unwrap();
    let missing = dir.path().join("nonexistent.toml");
    let result = run(Some(missing));
    assert!(result.is_err(), "whoami should fail when config is missing");
}

#[test]
fn test_fingerprint_short_key() {
    // Access via re-export isn't available; test indirectly through run().
    // A key shorter than 8 chars should still not panic.
    let dir = TempDir::new().unwrap();
    let cfg_path = make_config_with_key(&dir, Some("age1abc"));
    let result = run(Some(cfg_path));
    assert!(result.is_ok());
}
