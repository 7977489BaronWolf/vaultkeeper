pub mod edit;
pub mod init;
pub mod keygen;
pub mod list;
pub mod lock;
pub mod run;
pub mod status;
pub mod unlock;

#[cfg(test)]
mod edit_tests;
#[cfg(test)]
mod init_tests;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod status_tests;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Config;

#[derive(Parser)]
#[command(
    name = "vaultkeeper",
    about = "A lightweight CLI secrets manager using age encryption",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault in the current directory
    Init,
    /// Generate a new age keypair
    Keygen,
    /// Encrypt the .env file into the vault
    Lock,
    /// Decrypt the vault into a .env file
    Unlock,
    /// List all secret keys stored in the vault
    List,
    /// Run a command with secrets injected as environment variables
    Run {
        /// The command to execute
        #[arg(trailing_var_arg = true, required = true)]
        cmd: Vec<String>,
    },
    /// Edit a secret value by key
    Edit {
        /// The secret key to edit
        key: String,
    },
    /// Show vault and key status
    Status,
}

pub fn dispatch(cli: Cli, config: &Config) -> Result<()> {
    match cli.command {
        Commands::Init => init::run(config),
        Commands::Keygen => keygen::run(config),
        Commands::Lock => lock::run(config),
        Commands::Unlock => unlock::run(config),
        Commands::List => list::run(config),
        Commands::Run { cmd } => run::run(config, &cmd),
        Commands::Edit { key } => edit::run(config, &key),
        Commands::Status => status::run(config),
    }
}
