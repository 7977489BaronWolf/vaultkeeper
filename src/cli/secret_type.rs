use crate::store::secret_type::{self, SecretType, builtin_types};
use std::collections::HashMap;
use std::str::FromStr;

/// Handles the `vaultkeeper type set <key> <type>` subcommand.
pub fn cmd_set_type(
    store: &mut HashMap<String, String>,
    key: &str,
    type_str: &str,
) -> Result<(), String> {
    if !store.contains_key(key) {
        return Err(format!("Key '{}' does not exist in the store.", key));
    }
    let secret_type = SecretType::from_str(type_str)
        .map_err(|e| format!("Invalid secret type: {}", e))?;
    secret_type::annotate(store, key, &secret_type);
    println!("Set type of '{}' to '{}'.", key, secret_type);
    Ok(())
}

/// Handles the `vaultkeeper type get <key>` subcommand.
pub fn cmd_get_type(store: &HashMap<String, String>, key: &str) -> Result<(), String> {
    if !store.contains_key(key) {
        return Err(format!("Key '{}' does not exist in the store.", key));
    }
    let t = secret_type::get_type(store, key);
    println!("{}: {}", key, t);
    Ok(())
}

/// Handles the `vaultkeeper type list` subcommand — lists all keys with their types.
pub fn cmd_list_types(store: &HashMap<String, String>) {
    let mut keys: Vec<&String> = store
        .keys()
        .filter(|k| !k.starts_with("__type__"))
        .collect();
    keys.sort();

    if keys.is_empty() {
        println!("No secrets found.");
        return;
    }

    println!("{:<30} {}", "KEY", "TYPE");
    println!("{}", "-".repeat(45));
    for key in keys {
        let t = secret_type::get_type(store, key);
        println!("{:<30} {}", key, t);
    }
}

/// Handles the `vaultkeeper type available` subcommand.
pub fn cmd_list_available_types() {
    println!("Available built-in secret types:");
    for t in builtin_types() {
        println!("  {}", t);
    }
    println!("  custom:<name>  (e.g. custom:oauth_secret)");
}
