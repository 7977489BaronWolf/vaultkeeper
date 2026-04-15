use anyhow::{Context, Result};
use clap::Args;

use crate::config::Config;
use crate::store::VaultStore;

use super::env::load_env_vars;

#[derive(Args, Debug)]
pub struct GetArgs {
    /// Name of the vault entry
    pub name: String,

    /// Specific key to retrieve (prints all keys if omitted)
    pub key: Option<String>,

    /// Output as KEY=VALUE pairs (default: value only)
    #[arg(short, long)]
    pub export: bool,
}

pub fn handle_get(args: &GetArgs) -> Result<()> {
    let config = Config::load().context("Failed to load vaultkeeper config")?;
    let store = VaultStore::load(&config).context("Failed to load vault store")?;

    let vars = load_env_vars(&config, &store, &args.name)?;

    match &args.key {
        Some(key) => {
            let value = vars
                .get(key)
                .with_context(|| format!("Key '{}' not found in '{}'", key, args.name))?;
            if args.export {
                println!("{}={}", key, value);
            } else {
                println!("{}", value);
            }
        }
        None => {
            let mut keys: Vec<&String> = vars.keys().collect();
            keys.sort();
            for k in keys {
                if args.export {
                    let v = &vars[k];
                    if v.contains(' ') {
                        println!("export {}=\"{}\"", k, v);
                    } else {
                        println!("export {}={}", k, v);
                    }
                } else {
                    println!("{}", k);
                }
            }
        }
    }

    Ok(())
}
