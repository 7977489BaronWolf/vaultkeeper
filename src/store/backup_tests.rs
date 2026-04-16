use super::backup::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_backup_creates_file() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("secrets.age");
    fs::write(&vault, b"encrypted-data").unwrap();

    let backup = backup_vault(&vault).unwrap();
    assert!(backup.exists());
    assert_eq!(fs::read(&backup).unwrap(), b"encrypted-data");
}

#[test]
fn test_backup_nonexistent_vault_errors() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("missing.age");
    let result = backup_vault(&vault);
    assert!(result.is_err());
}

#[test]
fn test_list_backups_empty_when_none() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("secrets.age");
    let backups = list_backups(&vault).unwrap();
    assert!(backups.is_empty());
}

#[test]
fn test_list_backups_returns_sorted() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("secrets.age");
    fs::write(&vault, b"v1").unwrap();

    let b1 = backup_vault(&vault).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    fs::write(&vault, b"v2").unwrap();
    let b2 = backup_vault(&vault).unwrap();

    let backups = list_backups(&vault).unwrap();
    assert_eq!(backups.len(), 2);
    // newest first
    assert!(backups[0].file_name() >= backups[1].file_name());
    let _ = (b1, b2);
}

#[test]
fn test_restore_backup() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("secrets.age");
    fs::write(&vault, b"original").unwrap();

    let backup = backup_vault(&vault).unwrap();
    fs::write(&vault, b"modified").unwrap();

    restore_backup(&backup, &vault).unwrap();
    assert_eq!(fs::read(&vault).unwrap(), b"original");
}

#[test]
fn test_restore_missing_backup_errors() {
    let dir = tempdir().unwrap();
    let vault = dir.path().join("secrets.age");
    let fake_backup = dir.path().join("nope.age");
    let result = restore_backup(&fake_backup, &vault);
    assert!(result.is_err());
}
