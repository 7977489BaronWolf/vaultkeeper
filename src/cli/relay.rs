use crate::store::relay::RelayStore;
use std::collections::HashMap;

pub fn handle_relay_add(store: &mut RelayStore, source: &str, targets: Vec<String>) {
    match store.add_rule(source, targets) {
        Ok(_) => println!("Relay rule added: {} -> {} target(s)", source, store.get_targets(source).map(|t| t.len()).unwrap_or(0)),
        Err(e) => eprintln!("Error adding relay rule: {}", e),
    }
}

pub fn handle_relay_remove(store: &mut RelayStore, source: &str) {
    if store.remove_rule(source) {
        println!("Relay rule removed for: {}", source);
    } else {
        eprintln!("No relay rule found for: {}", source);
    }
}

pub fn handle_relay_list(store: &RelayStore) {
    let rules = store.list();
    if rules.is_empty() {
        println!("No relay rules defined.");
        return;
    }
    println!("{:<30} {}", "SOURCE", "TARGETS");
    println!("{}", "-".repeat(60));
    for rule in rules {
        println!("{:<30} {}", rule.source, rule.targets.join(", "));
    }
}

pub fn handle_relay_resolve(store: &RelayStore, secrets: &HashMap<String, String>) {
    let resolved = store.resolve(secrets);
    if resolved.is_empty() {
        println!("No relay propagations resolved.");
        return;
    }
    println!("Resolved relay propagations:");
    let mut keys: Vec<&String> = resolved.keys().collect();
    keys.sort();
    for key in keys {
        println!("  {} = {}", key, resolved[key]);
    }
}
