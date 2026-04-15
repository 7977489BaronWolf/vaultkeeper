mod cli;
mod config;
mod crypto;
mod store;

#[cfg(test)]
mod config_tests;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Commands::Init => cli::init::run_init(&config)?,
        Commands::Keygen => cli::keygen::run_keygen(&config)?,
        Commands::Lock => cli::lock::run_lock(&config)?,
        Commands::Unlock => cli::unlock::run_unlock(&config)?,
        Commands::List => cli::list::run_list(&config)?,
        Commands::Run { cmd } => cli::run::run_with_secrets(&config, &cmd)?,
        Commands::Edit => cli::edit::run_edit(&config)?,
        Commands::Status => cli::status::run_status(&config)?,
        Commands::Rotate => cli::rotate::run_rotate(&config)?,
        Commands::Whoami => cli::whoami::run_whoami(&config)?,
        Commands::Env => cli::env::run_env(&config)?,
        Commands::Get { key } => cli::get::run_get(&config, &key)?,
        Commands::Set { key, value } => cli::set::run_set(&config, &key, &value)?,
        Commands::Delete { key } => cli::delete::run_delete(&config, &key)?,
        Commands::Import { path } => cli::import::run_import(&config, &path)?,
        Commands::Export { path } => cli::export::run_export(&config, &path)?,
        Commands::Unset { key } => cli::unset::run_unset(&config, &key)?,
        Commands::Copy { key } => cli::copy::run_copy(&config, &key)?,
        Commands::Diff { target_a, target_b } => {
            cli::diff::run_diff(&config, &target_a, target_b.as_deref())?
        }
    }

    Ok(())
}
