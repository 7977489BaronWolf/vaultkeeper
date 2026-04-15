use std::env;
use std::fs;
use std::process::Command;
use anyhow::{Context, Result};
use tempfile::NamedTempFile;
use std::io::Write;

use crate::store::VaultStore;
use crate::crypto::{decrypt_file, encrypt_file};

/// Opens the decrypted vault contents in the user's $EDITOR,
/// then re-encrypts the (possibly modified) contents on save.
pub fn handle_edit(vault_path: &str, key_path: &str) -> Result<()> {
    let store = VaultStore::new(vault_path);

    // Decrypt current contents into a temp file
    let decrypted = decrypt_file(vault_path, key_path)
        .context("Failed to decrypt vault for editing")?;

    let mut tmp = NamedTempFile::new().context("Failed to create temp file")?;
    tmp.write_all(&decrypted).context("Failed to write temp file")?;
    let tmp_path = tmp.path().to_path_buf();

    // Capture mtime before opening editor
    let mtime_before = fs::metadata(&tmp_path)
        .and_then(|m| m.modified())
        .context("Failed to read temp file metadata")?;

    // Open editor
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let status = Command::new(&editor)
        .arg(&tmp_path)
        .status()
        .with_context(|| format!("Failed to launch editor: {}", editor))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    // Check if file was modified
    let mtime_after = fs::metadata(&tmp_path)
        .and_then(|m| m.modified())
        .context("Failed to read temp file metadata after edit")?;

    if mtime_before == mtime_after {
        println!("No changes made.");
        return Ok(());
    }

    // Re-encrypt modified contents
    let modified = fs::read(&tmp_path).context("Failed to read modified temp file")?;
    encrypt_file(&modified, vault_path, key_path)
        .context("Failed to re-encrypt vault after editing")?;

    println!("Vault updated successfully.");
    Ok(())
}
