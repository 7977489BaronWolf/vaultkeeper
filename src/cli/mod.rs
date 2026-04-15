pub mod edit;
#[cfg(test)]
mod edit_tests;

pub mod init;
#[cfg(test)]
mod init_tests;

pub mod keygen;

pub mod list;

pub mod lock;

pub mod rotate;
#[cfg(test)]
mod rotate_tests;

pub mod run;
#[cfg(test)]
mod run_tests;

pub mod status;
#[cfg(test)]
mod status_tests;

pub mod unlock;

pub mod whoami;
#[cfg(test)]
mod whoami_tests;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "vaultkeeper",
    about = "A lightweight CLI secrets manager using age encryption",
    version
)]
pub struct Cli {
    /// Path to the vaultkeeper config file
    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise a new vault in the current directory
    Init,
    /// Generate a new age keypair
    Keygen,
    /// Encrypt the .env file into the vault
    Lock,
    /// Decrypt the vault into a .env file
    Unlock,
    /// List secrets stored in the vault
    List,
    /// Run a command with secrets injected as environment variables
    Run {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Edit secrets interactively
    Edit,
    /// Show vault status
    Status,
    /// Rotate the encryption key
    Rotate,
    /// Show the current identity and key information
    Whoami,
}

pub fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init => init::run(cli.config),
        Commands::Keygen => keygen::run(cli.config),
        Commands::Lock => lock::run(cli.config),
        Commands::Unlock => unlock::run(cli.config),
        Commands::List => list::run(cli.config),
        Commands::Run { args } => run::run(cli.config, args),
        Commands::Edit => edit::run(cli.config),
        Commands::Status => status::run(cli.config),
        Commands::Rotate => rotate::run(cli.config),
        Commands::Whoami => whoami::run(cli.config),
    }
}
