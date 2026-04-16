use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

/// Creates a timestamped backup of a vault file.
pub fn backup_vault(vault_path: &Path) -> anyhow::Result<PathBuf> {
    if !vault_path.exists() {
        anyhow::bail!("Vault file not found: {}", vault_path.display());
    }

    let backup_dir = vault_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(".vaultkeeper_backups");

    fs::create_dir_all(&backup_dir)?;

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let stem = vault_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("vault");
    let ext = vault_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("age");

    let backup_name = format!("{}.{}.{}", stem, timestamp, ext);
    let backup_path = backup_dir.join(&backup_name);

    fs::copy(vault_path, &backup_path)?;
    Ok(backup_path)
}

/// Lists all backups for a given vault file, sorted newest first.
pub fn list_backups(vault_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let backup_dir = vault_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(".vaultkeeper_backups");

    if !backup_dir.exists() {
        return Ok(vec![]);
    }

    let stem = vault_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("vault");

    let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(stem))
                .unwrap_or(false)
        })
        .collect();

    backups.sort_by(|a, b| b.cmp(a));
    Ok(backups)
}

/// Restores a vault from a backup path.
pub fn restore_backup(backup_path: &Path, vault_path: &Path) -> anyhow::Result<()> {
    if !backup_path.exists() {
        anyhow::bail!("Backup not found: {}", backup_path.display());
    }
    fs::copy(backup_path, vault_path)?;
    Ok(())
}
