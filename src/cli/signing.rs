use crate::store::signing::SigningStore;
use std::path::Path;

pub fn handle_sign(store: &mut SigningStore, key: &str, value: &str, signer: &str) {
    let entry = store.sign(key, value, signer);
    println!("Signed '{}' by '{}' at {}", key, entry.signer, entry.signed_at);
    println!("Signature: {}", entry.signature);
}

pub fn handle_verify(store: &SigningStore, key: &str, value: &str) {
    if store.verify(key, value) {
        println!("✓ Signature valid for '{}'", key);
    } else {
        eprintln!("✗ Signature invalid or missing for '{}'", key);
        std::process::exit(1);
    }
}

pub fn handle_show_signature(store: &SigningStore, key: &str) {
    match store.get_signature(key) {
        Some(entry) => {
            println!("Key:       {}", entry.key);
            println!("Signer:    {}", entry.signer);
            println!("Signature: {}", entry.signature);
            println!("Signed At: {}", entry.signed_at);
        }
        None => {
            eprintln!("No signature found for '{}'", key);
            std::process::exit(1);
        }
    }
}

pub fn handle_list_signed(store: &SigningStore) {
    let entries = store.list_signed();
    if entries.is_empty() {
        println!("No signed entries.");
        return;
    }
    println!("{:<20} {:<16} {}", "KEY", "SIGNER", "SIGNED AT");
    println!("{}", "-".repeat(56));
    let mut sorted: Vec<_> = entries.into_iter().collect();
    sorted.sort_by(|a, b| a.key.cmp(&b.key));
    for entry in sorted {
        println!("{:<20} {:<16} {}", entry.key, entry.signer, entry.signed_at);
    }
}

pub fn handle_unsign(store: &mut SigningStore, key: &str) {
    if store.remove(key) {
        println!("Removed signature for '{}'", key);
    } else {
        eprintln!("No signature found for '{}'", key);
        std::process::exit(1);
    }
}

pub fn load_signing_store(path: &Path) -> SigningStore {
    if path.exists() {
        let data = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        SigningStore::new()
    }
}

pub fn save_signing_store(store: &SigningStore, path: &Path) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(store)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(path, data)
}
