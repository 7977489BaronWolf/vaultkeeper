use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub env: String,
    pub created_at: DateTime<Utc>,
    pub label: Option<String>,
    pub entries: HashMap<String, String>,
}

impl Snapshot {
    pub fn new(env: &str, entries: HashMap<String, String>, label: Option<String>) -> Self {
        let id = format!("{:x}", md5_hash(&format!("{}{}", env, Utc::now().timestamp_nanos_opt().unwrap_or(0))));
        Self {
            id,
            env: env.to_string(),
            created_at: Utc::now(),
            label,
            entries,
        }
    }
}

fn md5_hash(input: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

pub fn save_snapshot(snapshots_dir: &Path, snapshot: &Snapshot) -> anyhow::Result<()> {
    fs::create_dir_all(snapshots_dir)?;
    let file_path = snapshots_dir.join(format!("{}.json", snapshot.id));
    let content = serde_json::to_string_pretty(snapshot)?;
    fs::write(file_path, content)?;
    Ok(())
}

pub fn load_snapshots(snapshots_dir: &Path, env: &str) -> anyhow::Result<Vec<Snapshot>> {
    if !snapshots_dir.exists() {
        return Ok(vec![]);
    }
    let mut snapshots = vec![];
    for entry in fs::read_dir(snapshots_dir)? {
        let entry = entry?;
        let content = fs::read_to_string(entry.path())?;
        if let Ok(snap) = serde_json::from_str::<Snapshot>(&content) {
            if snap.env == env {
                snapshots.push(snap);
            }
        }
    }
    snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(snapshots)
}

pub fn delete_snapshot(snapshots_dir: &Path, id: &str) -> anyhow::Result<()> {
    let file_path = snapshots_dir.join(format!("{}.json", id));
    if file_path.exists() {
        fs::remove_file(file_path)?;
    }
    Ok(())
}
