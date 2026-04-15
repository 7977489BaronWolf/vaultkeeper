use std::path::Path;
use anyhow::{Context, Result};
use crate::store::VaultStore;

pub fn run(vault_dir: &Path) -> Result<()> {
    let store = VaultStore::load(vault_dir)
        .context("Failed to load vault store. Have you initialized a vault here?")?;

    let entries = store.list_entries();

    if entries.is_empty() {
        println!("No secrets stored in vault.");
        return Ok(());
    }

    println!("Stored secrets ({}):", entries.len());
    println!("{:-<40}", "");
    for entry in &entries {
        println!("  {}", entry);
    }
    println!("{:-<40}", "");

    Ok(())
}
