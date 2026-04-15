pub mod keygen;
pub mod lock;
pub mod unlock;
pub mod list;
pub mod run;
pub mod edit;
pub mod init;
pub mod status;
pub mod rotate;
pub mod whoami;
pub mod env;
pub mod get;
pub mod set;
pub mod delete;
pub mod import;
pub mod export;

#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod edit_tests;
#[cfg(test)]
mod init_tests;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod rotate_tests;
#[cfg(test)]
mod whoami_tests;
#[cfg(test)]
mod env_tests;
#[cfg(test)]
mod get_tests;
#[cfg(test)]
mod set_tests;
#[cfg(test)]
mod delete_tests;
#[cfg(test)]
mod import_tests;
#[cfg(test)]
mod export_tests;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "vaultkeeper", about = "A lightweight CLI secrets manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault
    Init { name: Option<String> },
    /// Generate a new age keypair
    Keygen,
    /// Encrypt the vault
    Lock { vault: Option<String> },
    /// Decrypt the vault
    Unlock { vault: Option<String> },
    /// List all vaults
    List,
    /// Run a command with secrets injected as env vars
    Run { vault: Option<String>, #[arg(last = true)] cmd: Vec<String> },
    /// Edit secrets in the vault
    Edit { vault: Option<String> },
    /// Show vault status
    Status { vault: Option<String> },
    /// Rotate encryption key
    Rotate { vault: Option<String> },
    /// Show current identity
    Whoami,
    /// Print secrets as env exports
    Env { vault: Option<String> },
    /// Get a secret value
    Get { key: String, vault: Option<String> },
    /// Set a secret value
    Set { key: String, value: String, vault: Option<String> },
    /// Delete a secret
    Delete { key: String, vault: Option<String> },
    /// Import secrets from a .env file
    Import {
        /// Path to the .env file to import
        file: PathBuf,
        /// Target vault name
        #[arg(short, long)]
        vault: Option<String>,
        /// Overwrite existing keys
        #[arg(short, long)]
        force: bool,
    },
    /// Export secrets to a .env file
    Export {
        /// Output file path
        output: PathBuf,
        /// Source vault name
        #[arg(short, long)]
        vault: Option<String>,
        /// Overwrite output file if it exists
        #[arg(long)]
        overwrite: bool,
    },
}
