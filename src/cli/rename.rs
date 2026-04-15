use crate::config::Config;
use crate::store::Store;
use anyhow::{bail, Context, Result};
use std::path::PathBuf;

pub fn run(env: &str, new_name: &str, config_path: Option<PathBuf>) -> Result<()> {
    let config = Config::load(config_path.as_deref())
        .context("Failed to load vaultkeeper config")?;

    if env == new_name {
        bail!("Source and destination environment names are the same: '{}'", env);
    }

    let mut store = Store::load(&config.store_path)
        .context("Failed to load store")?;

    if !store.has_env(env) {
        bail!("Environment '{}' does not exist", env);
    }

    if store.has_env(new_name) {
        bail!(
            "Environment '{}' already exists. Remove it first or choose a different name.",
            new_name
        );
    }

    store
        .rename_env(env, new_name)
        .with_context(|| format!("Failed to rename '{}' to '{}'", env, new_name))?;

    store
        .save(&config.store_path)
        .context("Failed to save store after rename")?;

    println!("Renamed environment '{}' → '{}'", env, new_name);
    Ok(())
}
