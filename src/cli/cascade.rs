use crate::store::cascade::CascadeChain;
use std::collections::HashMap;

/// Run the `cascade` subcommand: resolve a key or show the full merged env
/// across an ordered list of namespaces/profiles.
pub fn run_cascade(
    layers: &[String],
    key: Option<&str>,
    stores: &HashMap<String, HashMap<String, String>>,
) -> anyhow::Result<()> {
    if layers.is_empty() {
        anyhow::bail!("At least one layer must be specified.");
    }

    let chain = CascadeChain::new(layers.to_vec());

    match key {
        Some(k) => {
            match chain.resolve(k, stores) {
                Some((layer, value)) => {
                    println!("{}  (from: {})", value, layer);
                }
                None => {
                    eprintln!("Key '{}' not found in any layer: {}", k, layers.join(" -> "));
                    std::process::exit(1);
                }
            }
        }
        None => {
            let flat = chain.flatten(stores);
            let mut pairs: Vec<(&String, &String)> = flat.iter().collect();
            pairs.sort_by_key(|(k, _)| k.as_str());
            for (k, v) in pairs {
                println!("{}={}", k, v);
            }
        }
    }

    Ok(())
}

/// Print the cascade resolution order and which layer each key comes from.
pub fn run_cascade_explain(
    layers: &[String],
    stores: &HashMap<String, HashMap<String, String>>,
) -> anyhow::Result<()> {
    if layers.is_empty() {
        anyhow::bail!("At least one layer must be specified.");
    }

    let chain = CascadeChain::new(layers.to_vec());
    println!("Resolution order: {}", layers.join(" -> "));
    println!();

    let keys = chain.all_keys(stores);
    for key in &keys {
        if let Some((layer, value)) = chain.resolve(key, stores) {
            println!("{:<30} = {:<40} [{}]", key, value, layer);
        }
    }

    Ok(())
}
