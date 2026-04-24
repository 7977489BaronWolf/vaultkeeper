use crate::store::namespace::{Namespace, NamespaceStore};

/// Handles the `namespace add <name> [--description <desc>]` command.
pub fn cmd_namespace_add(
    store: &mut NamespaceStore,
    name: &str,
    description: Option<&str>,
) -> Result<(), String> {
    let ns = Namespace::new(name, description);
    store.add_namespace(ns)?;
    println!("Namespace '{}' created.", name);
    Ok(())
}

/// Handles the `namespace remove <name>` command.
pub fn cmd_namespace_remove(store: &mut NamespaceStore, name: &str) -> Result<(), String> {
    store.remove_namespace(name)?;
    println!("Namespace '{}' removed.", name);
    Ok(())
}

/// Handles the `namespace list` command.
pub fn cmd_namespace_list(store: &NamespaceStore) {
    let namespaces = store.list_namespaces();
    if namespaces.is_empty() {
        println!("No namespaces defined.");
        return;
    }
    println!("{:<20} {}", "NAME", "DESCRIPTION");
    println!("{}", "-".repeat(45));
    for ns in namespaces {
        let desc = ns.description.as_deref().unwrap_or("-");
        println!("{:<20} {}", ns.name, desc);
    }
}

/// Handles the `namespace assign <key> <namespace>` command.
pub fn cmd_namespace_assign(
    store: &mut NamespaceStore,
    key: &str,
    namespace: &str,
) -> Result<(), String> {
    store.assign(key, namespace)?;
    println!("Key '{}' assigned to namespace '{}'.", key, namespace);
    Ok(())
}

/// Handles the `namespace keys <namespace>` command.
pub fn cmd_namespace_keys(store: &NamespaceStore, namespace: &str) {
    let mut keys = store.keys_in_namespace(namespace);
    if keys.is_empty() {
        println!("No keys in namespace '{}'.", namespace);
        return;
    }
    keys.sort();
    println!("Keys in namespace '{}':", namespace);
    for key in keys {
        println!("  {}", key);
    }
}
