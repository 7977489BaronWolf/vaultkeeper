#[cfg(test)]
mod tests {
    use super::super::audit::*;

    #[test]
    fn test_audit_action_display_created() {
        assert_eq!(AuditAction::Created.to_string(), "vault created");
    }

    #[test]
    fn test_audit_action_display_locked_unlocked() {
        assert_eq!(AuditAction::Locked.to_string(), "vault locked");
        assert_eq!(AuditAction::Unlocked.to_string(), "vault unlocked");
    }

    #[test]
    fn test_audit_action_display_secret_set() {
        let action = AuditAction::SecretSet("API_KEY".to_string());
        assert_eq!(action.to_string(), "secret set: API_KEY");
    }

    #[test]
    fn test_audit_action_display_secret_deleted() {
        let action = AuditAction::SecretDeleted("DB_PASS".to_string());
        assert_eq!(action.to_string(), "secret deleted: DB_PASS");
    }

    #[test]
    fn test_audit_action_display_secret_renamed() {
        let action = AuditAction::SecretRenamed {
            from: "OLD_KEY".to_string(),
            to: "NEW_KEY".to_string(),
        };
        assert_eq!(action.to_string(), "secret renamed: OLD_KEY -> NEW_KEY");
    }

    #[test]
    fn test_audit_action_display_imported_exported() {
        assert_eq!(AuditAction::Imported(".env".to_string()).to_string(), "imported from: .env");
        assert_eq!(AuditAction::Exported("out.env".to_string()).to_string(), "exported to: out.env");
    }

    #[test]
    fn test_audit_entry_new_has_nonzero_timestamp() {
        let entry = AuditEntry::new(AuditAction::Created);
        assert!(entry.timestamp > 0);
    }

    #[test]
    fn test_append_audit_log() {
        let mut log = Vec::new();
        append_audit_log(&mut log, AuditAction::Created);
        append_audit_log(&mut log, AuditAction::Locked);
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].action, AuditAction::Created);
        assert_eq!(log[1].action, AuditAction::Locked);
    }

    #[test]
    fn test_format_audit_log_empty() {
        let log: Vec<AuditEntry> = Vec::new();
        assert_eq!(format_audit_log(&log), "No audit entries found.");
    }

    #[test]
    fn test_format_audit_log_nonempty() {
        let mut log = Vec::new();
        append_audit_log(&mut log, AuditAction::KeyRotated);
        let output = format_audit_log(&log);
        assert!(output.contains("key rotated"));
    }

    #[test]
    fn test_snapshot_actions_display() {
        let created = AuditAction::SnapshotCreated("snap-1".to_string());
        let restored = AuditAction::SnapshotRestored("snap-1".to_string());
        assert_eq!(created.to_string(), "snapshot created: snap-1");
        assert_eq!(restored.to_string(), "snapshot restored: snap-1");
    }
}
