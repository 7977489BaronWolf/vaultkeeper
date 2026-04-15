pub mod copy;
pub mod delete;
pub mod edit;
pub mod env;
pub mod export;
pub mod get;
pub mod import;
pub mod init;
pub mod keygen;
pub mod list;
pub mod lock;
pub mod rotate;
pub mod run;
pub mod set;
pub mod status;
pub mod unset;
pub mod unlock;
pub mod whoami;
pub mod diff;

#[cfg(test)]
mod copy_tests;
#[cfg(test)]
mod delete_tests;
#[cfg(test)]
mod edit_tests;
#[cfg(test)]
mod env_tests;
#[cfg(test)]
mod export_tests;
#[cfg(test)]
mod get_tests;
#[cfg(test)]
mod import_tests;
#[cfg(test)]
mod init_tests;
#[cfg(test)]
mod rotate_tests;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod set_tests;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod unset_tests;
#[cfg(test)]
mod whoami_tests;
#[cfg(test)]
mod diff_tests;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vaultkeeper", version, about = "A lightweight CLI secrets manager")]
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
    /// Encrypt the env file (lock the vault)
    Lock,
    /// Decrypt the vault (unlock)
    Unlock,
    /// List all secret keys
    List,
    /// Run a command with secrets injected
    Run {
        #[arg(trailing_var_arg = true)]
        cmd: Vec<String>,
    },
    /// Edit the vault in $EDITOR
    Edit,
    /// Show vault status
    Status,
    /// Rotate the encryption key
    Rotate,
    /// Show current identity
    Whoami,
    /// Print secrets as env exports
    Env,
    /// Get a secret value by key
    Get { key: String },
    /// Set a secret key-value pair
    Set { key: String, value: String },
    /// Delete a secret key
    Delete { key: String },
    /// Import secrets from a file
    Import { path: String },
    /// Export secrets to a file
    Export { path: String },
    /// Unset (remove) a key from the vault
    Unset { key: String },
    /// Copy a secret value to clipboard
    Copy { key: String },
    /// Show diff between vault and another file or vault
    Diff {
        /// First file or vault to compare
        target_a: String,
        /// Second file or vault (defaults to plaintext .env)
        target_b: Option<String>,
    },
}
