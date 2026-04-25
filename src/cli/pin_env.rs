use crate::store::pin_env::{diff_pinned, pin_env};

/// Handle `vaultkeeper pin-env <name>` — pins the current decrypted env.
pub fn handle_pin_env(name: &str, entries: &[(String, String)]) {
    match pin_env(name, entries) {
        Ok(pinned) => {
            println!("Pinned {} variable(s) as '{}'", pinned.vars.len(), pinned.name);
        }
        Err(e) => {
            eprintln!("Error pinning env: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `vaultkeeper pin-env diff <name>` — shows drift from a saved pin.
pub fn handle_pin_env_diff(
    name: &str,
    entries: &[(String, String)],
    current: &[(String, String)],
) {
    let pinned = match pin_env(name, entries) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading pin '{}': {}", name, e);
            std::process::exit(1);
        }
    };

    let (added, removed, changed) = diff_pinned(&pinned, current);

    if added.is_empty() && removed.is_empty() && changed.is_empty() {
        println!("No drift detected from pin '{}'.", name);
        return;
    }

    if !added.is_empty() {
        println!("Added since pin:");
        for k in &added {
            println!("  + {}", k);
        }
    }
    if !removed.is_empty() {
        println!("Removed since pin:");
        for k in &removed {
            println!("  - {}", k);
        }
    }
    if !changed.is_empty() {
        println!("Changed since pin:");
        for k in &changed {
            println!("  ~ {}", k);
        }
    }
}
