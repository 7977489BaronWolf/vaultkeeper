mod cli;
mod crypto;
mod store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { output } => {
            cli::keygen::generate_keypair(&output)?;
        }
        Commands::Lock { env_file, recipient } => {
            cli::lock::lock_env_file(&env_file, &recipient)?;
        }
        Commands::Unlock { output, identity } => {
            cli::unlock::unlock_env_file(&output, &identity)?;
        }
        Commands::List => {
            cli::list::list_keys()?;
        }
        Commands::Run { vault, identity, command } => {
            cli::run::run_with_secrets(&vault, &identity, &command)?;
        }
    }

    Ok(())
}
