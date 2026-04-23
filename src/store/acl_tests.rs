#[cfg(test)]
mod tests {
    use super::super::acl::{Acl, Permission};

    #[test]
    fn test_grant_and_check_permission() {
        let mut acl = Acl::new();
        acl.grant("DB_*", "alice", Permission::Read);
        assert!(acl.is_allowed("DB_HOST", "alice", &Permission::Read));
        assert!(!acl.is_allowed("DB_HOST", "alice", &Permission::Write));
        assert!(!acl.is_allowed("API_KEY", "alice", &Permission::Read));
    }

    #[test]
    fn test_revoke_permission() {
        let mut acl = Acl::new();
        acl.grant("*", "bob", Permission::Write);
        assert!(acl.is_allowed("ANY_KEY", "bob", &Permission::Write));
        acl.revoke("*", "bob", &Permission::Write);
        assert!(!acl.is_allowed("ANY_KEY", "bob", &Permission::Write));
    }

    #[test]
    fn test_wildcard_matches_all_keys() {
        let mut acl = Acl::new();
        acl.grant("*", "admin", Permission::Delete);
        assert!(acl.is_allowed("SECRET", "admin", &Permission::Delete));
        assert!(acl.is_allowed("DB_PASS", "admin", &Permission::Delete));
    }

    #[test]
    fn test_exact_pattern_match() {
        let mut acl = Acl::new();
        acl.grant("STRIPE_KEY", "svc-billing", Permission::Read);
        assert!(acl.is_allowed("STRIPE_KEY", "svc-billing", &Permission::Read));
        assert!(!acl.is_allowed("STRIPE_SECRET", "svc-billing", &Permission::Read));
    }

    #[test]
    fn test_multiple_identities_same_pattern() {
        let mut acl = Acl::new();
        acl.grant("AWS_*", "dev", Permission::Read);
        acl.grant("AWS_*", "ops", Permission::Write);
        assert!(acl.is_allowed("AWS_SECRET", "dev", &Permission::Read));
        assert!(!acl.is_allowed("AWS_SECRET", "dev", &Permission::Write));
        assert!(acl.is_allowed("AWS_SECRET", "ops", &Permission::Write));
    }

    #[test]
    fn test_list_entries() {
        let mut acl = Acl::new();
        acl.grant("DB_*", "alice", Permission::Read);
        acl.grant("DB_*", "alice", Permission::Write);
        let entries = acl.list();
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().any(|(p, i, r)| p == "DB_*" && i == "alice" && r == "read"));
        assert!(entries.iter().any(|(p, i, r)| p == "DB_*" && i == "alice" && r == "write"));
    }

    #[test]
    fn test_suffix_glob_pattern() {
        let mut acl = Acl::new();
        acl.grant("*_KEY", "ci", Permission::Read);
        assert!(acl.is_allowed("STRIPE_KEY", "ci", &Permission::Read));
        assert!(acl.is_allowed("API_KEY", "ci", &Permission::Read));
        assert!(!acl.is_allowed("DB_HOST", "ci", &Permission::Read));
    }

    #[test]
    fn test_unknown_identity_denied() {
        let mut acl = Acl::new();
        acl.grant("*", "known", Permission::Read);
        assert!(!acl.is_allowed("SECRET", "unknown", &Permission::Read));
    }
}
