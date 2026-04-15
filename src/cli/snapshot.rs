use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::config::Config;
use crate::store::snapshot::{delete_snapshot, load_snapshots, save_snapshot, Snapshot};
use crate::store::mod_plain::read_plain;

pub fn handle_snapshot_save(env: &str, label: Option<String>, config: &Config) -> Result<()> {
    let plain_path = config.plain_path(env);
    let entries = read_plain(&plain_path)
        .with_context(|| format!("Failed to read env '{}'. Is it unlocked?", env))?;
    let snapshot = Snapshot::new(env, entries, label.clone());
    let snapshots_dir = config.snapshots_dir();
    save_snapshot(&snapshots_dir, &snapshot)?;
    println!(
        "Snapshot '{}' saved for env '{}'{}.",
        snapshot.id,
        env,
        label.map(|l| format!(" ({})", l)).unwrap_or_default()
    );
    Ok(())
}

pub fn handle_snapshot_list(env: &str, config: &Config) -> Result<()> {
    let snapshots_dir = config.snapshots_dir();
    let snapshots = load_snapshots(&snapshots_dir, env)?;
    if snapshots.is_empty() {
        println!("No snapshots found for env '{}'.", env);
        return Ok(());
    }
    println!("Snapshots for '{}':", env);
    for snap in &snapshots {
        let label = snap.label.as_deref().unwrap_or("(no label)");
        println!("  {} | {} | {} keys | {}", &snap.id[..8], snap.created_at.format("%Y-%m-%d %H:%M:%S"), snap.entries.len(), label);
    }
    Ok(())
}

pub fn handle_snapshot_restore(env: &str, id: &str, config: &Config) -> Result<()> {
    let snapshots_dir = config.snapshots_dir();
    let snapshots = load_snapshots(&snapshots_dir, env)?;
    let snap = snapshots.iter().find(|s| s.id.starts_with(id))
        .with_context(|| format!("Snapshot '{}' not found for env '{}'.", id, env))?;
    let plain_path = config.plain_path(env);
    crate::store::mod_plain::write_plain(&plain_path, &snap.entries)?;
    println!("Restored snapshot '{}' to env '{}'.", &snap.id[..8], env);
    Ok(())
}

pub fn handle_snapshot_delete(env: &str, id: &str, config: &Config) -> Result<()> {
    let snapshots_dir = config.snapshots_dir();
    let snapshots = load_snapshots(&snapshots_dir, env)?;
    let snap = snapshots.iter().find(|s| s.id.starts_with(id))
        .with_context(|| format!("Snapshot '{}' not found for env '{}'.", id, env))?;
    delete_snapshot(&snapshots_dir, &snap.id)?;
    println!("Deleted snapshot '{}' from env '{}'.", &snap.id[..8], env);
    Ok(())
}
