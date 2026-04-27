use crate::store::remote::{RemoteConfig, RemoteStore};

pub fn handle_remote_add(store: &mut RemoteStore, name: &str, url: &str, token: Option<&str>) {
    let config = RemoteConfig {
        name: name.to_string(),
        url: url.to_string(),
        auth_token: token.map(|t| t.to_string()),
    };
    match store.add(config) {
        Ok(_) => println!("Remote '{}' added ({})", name, url),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_remote_remove(store: &mut RemoteStore, name: &str) {
    match store.remove(name) {
        Ok(cfg) => println!("Remote '{}' removed", cfg.name),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_remote_list(store: &RemoteStore) {
    let remotes = store.list();
    if remotes.is_empty() {
        println!("No remotes configured.");
        return;
    }
    println!("{:<20} {:<40} {}", "NAME", "URL", "AUTH");
    println!("{}", "-".repeat(72));
    for r in remotes {
        let auth = if r.auth_token.is_some() { "[token set]" } else { "[none]" };
        println!("{:<20} {:<40} {}", r.name, r.url, auth);
    }
}

pub fn handle_remote_set_token(store: &mut RemoteStore, name: &str, token: Option<&str>) {
    match store.update_token(name, token.map(|t| t.to_string())) {
        Ok(_) => {
            if token.is_some() {
                println!("Token updated for remote '{}'.", name);
            } else {
                println!("Token cleared for remote '{}'.", name);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_remote_show(store: &RemoteStore, name: &str) {
    match store.get(name) {
        Some(r) => {
            println!("Name:  {}", r.name);
            println!("URL:   {}", r.url);
            println!("Auth:  {}", r.auth_token.as_deref().map(|_| "[token set]").unwrap_or("[none]"));
        }
        None => eprintln!("Error: Remote '{}' not found", name),
    }
}
