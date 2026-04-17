use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum AuditAction {
    Created,
    Locked,
    Unlocked,
    KeyRotated,
    SecretSet(String),
    SecretDeleted(String),
    SecretRenamed { from: String, to: String },
    Imported(String),
    Exported(String),
    SnapshotCreated(String),
    SnapshotRestored(String),
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditAction::Created => write!(f, "vault created"),
            AuditAction::Locked => write!(f, "vault locked"),
            AuditAction::Unlocked => write!(f, "vault unlocked"),
            AuditAction::KeyRotated => write!(f, "key rotated"),
            AuditAction::SecretSet(k) => write!(f, "secret set: {}", k),
            AuditAction::SecretDeleted(k) => write!(f, "secret deleted: {}", k),
            AuditAction::SecretRenamed { from, to } => write!(f, "secret renamed: {} -> {}", from, to),
            AuditAction::Imported(src) => write!(f, "imported from: {}", src),
            AuditAction::Exported(dst) => write!(f, "exported to: {}", dst),
            AuditAction::SnapshotCreated(name) => write!(f, "snapshot created: {}", name),
            AuditAction::SnapshotRestored(name) => write!(f, "snapshot restored: {}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: AuditAction,
}

impl AuditEntry {
    pub fn new(action: AuditAction) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self { timestamp, action }
    }

    pub fn format_timestamp(&self) -> String {
        let secs = self.timestamp;
        let days = secs / 86400;
        let rem = secs % 86400;
        let hours = rem / 3600;
        let minutes = (rem % 3600) / 60;
        let seconds = rem % 60;
        format!("epoch+{}d {:02}:{:02}:{:02}", days, hours, minutes, seconds)
    }
}

impl fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.format_timestamp(), self.action)
    }
}

pub fn append_audit_log(log: &mut Vec<AuditEntry>, action: AuditAction) {
    log.push(AuditEntry::new(action));
}

pub fn format_audit_log(log: &[AuditEntry]) -> String {
    if log.is_empty() {
        return "No audit entries found.".to_string();
    }
    log.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns all audit entries whose action matches the given predicate.
pub fn filter_audit_log<'a, F>(log: &'a [AuditEntry], predicate: F) -> Vec<&'a AuditEntry>
where
    F: Fn(&AuditAction) -> bool,
{
    log.iter().filter(|e| predicate(&e.action)).collect()
}
