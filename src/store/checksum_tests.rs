use super::*;
use std::collections::HashMap;

fn sample_secrets() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("API_KEY".to_string(), "abc123".to_string());
    m.insert("DB_PASS".to_string(), "secret!".to_string());
    m
}

#[test]
fn test_checksum_is_deterministic() {
    let secrets = sample_secrets();
    let c1 = compute_checksum(&secrets);
    let c2 = compute_checksum(&secrets);
    assert_eq!(c1, c2);
}

#[test]
fn test_checksum_order_independent() {
    let mut a = HashMap::new();
    a.insert("FOO".to_string(), "1".to_string());
    a.insert("BAR".to_string(), "2".to_string());

    let mut b = HashMap::new();
    b.insert("BAR".to_string(), "2".to_string());
    b.insert("FOO".to_string(), "1".to_string());

    assert_eq!(compute_checksum(&a), compute_checksum(&b));
}

#[test]
fn test_checksum_changes_on_value_change() {
    let mut secrets = sample_secrets();
    let original = compute_checksum(&secrets);
    secrets.insert("API_KEY".to_string(), "different".to_string());
    let modified = compute_checksum(&secrets);
    assert_ne!(original, modified);
}

#[test]
fn test_checksum_changes_on_new_key() {
    let mut secrets = sample_secrets();
    let original = compute_checksum(&secrets);
    secrets.insert("NEW_KEY".to_string(), "val".to_string());
    let modified = compute_checksum(&secrets);
    assert_ne!(original, modified);
}

#[test]
fn test_verify_checksum_pass() {
    let secrets = sample_secrets();
    let checksum = compute_checksum(&secrets);
    assert!(verify_checksum(&secrets, &checksum));
}

#[test]
fn test_verify_checksum_fail() {
    let secrets = sample_secrets();
    assert!(!verify_checksum(&secrets, "deadbeef"));
}

#[test]
fn test_entry_checksum_deterministic() {
    let c1 = compute_entry_checksum("MY_KEY", "my_value");
    let c2 = compute_entry_checksum("MY_KEY", "my_value");
    assert_eq!(c1, c2);
}

#[test]
fn test_entry_checksum_differs_for_different_values() {
    let c1 = compute_entry_checksum("MY_KEY", "value_a");
    let c2 = compute_entry_checksum("MY_KEY", "value_b");
    assert_ne!(c1, c2);
}

#[test]
fn test_empty_secrets_checksum() {
    let empty: HashMap<String, String> = HashMap::new();
    let checksum = compute_checksum(&empty);
    assert!(!checksum.is_empty());
    assert!(verify_checksum(&empty, &checksum));
}
