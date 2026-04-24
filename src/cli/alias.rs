use crate::store::alias::AliasStore;

pub fn cmd_alias_set(store: &mut AliasStore, alias: &str, target: &str) {
    if store.has_cycle(alias, target) {
        eprintln!("Error: setting alias '{}' -> '{}' would create a cycle", alias, target);
        return;
    }
    match store.set(alias, target) {
        Ok(()) => println!("Alias '{}' -> '{}' set.", alias, target),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn cmd_alias_get(store: &AliasStore, alias: &str) {
    match store.get(alias) {
        Some(target) => println!("{} -> {}", alias, target),
        None => eprintln!("Alias '{}' not found.", alias),
    }
}

pub fn cmd_alias_remove(store: &mut AliasStore, alias: &str) {
    match store.remove(alias) {
        Ok(()) => println!("Alias '{}' removed.", alias),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn cmd_alias_resolve(store: &AliasStore, key: &str) {
    let resolved = store.resolve(key);
    if resolved == key {
        println!("{} (no alias)", key);
    } else {
        println!("{} -> {}", key, resolved);
    }
}

pub fn cmd_alias_list(store: &AliasStore) {
    let list = store.list();
    if list.is_empty() {
        println!("No aliases defined.");
        return;
    }
    println!("{:<30} {}", "ALIAS", "TARGET");
    println!("{}", "-".repeat(50));
    for (alias, target) in list {
        println!("{:<30} {}", alias, target);
    }
}
