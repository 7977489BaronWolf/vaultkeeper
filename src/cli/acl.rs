use crate::store::acl::{Acl, Permission};

fn parse_permission(s: &str) -> Result<Permission, String> {
    match s.to_lowercase().as_str() {
        "read" => Ok(Permission::Read),
        "write" => Ok(Permission::Write),
        "delete" => Ok(Permission::Delete),
        other => Err(format!("Unknown permission '{}'. Use: read, write, delete", other)),
    }
}

pub fn handle_acl_grant(acl: &mut Acl, pattern: &str, identity: &str, permission: &str) -> Result<(), String> {
    let perm = parse_permission(permission)?;
    acl.grant(pattern, identity, perm);
    println!("Granted '{}' on '{}' to '{}'", permission, pattern, identity);
    Ok(())
}

pub fn handle_acl_revoke(acl: &mut Acl, pattern: &str, identity: &str, permission: &str) -> Result<(), String> {
    let perm = parse_permission(permission)?;
    acl.revoke(pattern, identity, &perm);
    println!("Revoked '{}' on '{}' from '{}'", permission, pattern, identity);
    Ok(())
}

pub fn handle_acl_check(acl: &Acl, key: &str, identity: &str, permission: &str) -> Result<(), String> {
    let perm = parse_permission(permission)?;
    if acl.is_allowed(key, identity, &perm) {
        println!("ALLOWED: '{}' has '{}' on '{}'", identity, permission, key);
    } else {
        println!("DENIED: '{}' does not have '{}' on '{}'", identity, permission, key);
    }
    Ok(())
}

pub fn handle_acl_list(acl: &Acl) {
    let entries = acl.list();
    if entries.is_empty() {
        println!("No ACL entries defined.");
        return;
    }
    println!("{:<20} {:<20} {}", "Pattern", "Identity", "Permission");
    println!("{}", "-".repeat(52));
    for (pattern, identity, permission) in &entries {
        println!("{:<20} {:<20} {}", pattern, identity, permission);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::acl::Acl;

    #[test]
    fn test_handle_grant_and_check() {
        let mut acl = Acl::new();
        handle_acl_grant(&mut acl, "DB_*", "alice", "read").unwrap();
        assert!(acl.is_allowed("DB_HOST", "alice", &Permission::Read));
    }

    #[test]
    fn test_handle_invalid_permission() {
        let mut acl = Acl::new();
        let result = handle_acl_grant(&mut acl, "*", "alice", "fly");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown permission"));
    }

    #[test]
    fn test_handle_revoke() {
        let mut acl = Acl::new();
        handle_acl_grant(&mut acl, "*", "bob", "write").unwrap();
        handle_acl_revoke(&mut acl, "*", "bob", "write").unwrap();
        assert!(!acl.is_allowed("KEY", "bob", &Permission::Write));
    }
}
