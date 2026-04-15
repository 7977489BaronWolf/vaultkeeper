pub mod keygen;
pub mod lock;
pub mod unlock;
pub mod list;
pub mod run;

#[cfg(test)]
mod run_tests;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vaultkeeper", about = "A lightweight CLI secrets manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new age keypair
    Keygen {
        #[arg(short, long, default_value = "./vault.key")]
        output: String,
    },
    /// Encrypt an env file into the vault
    Lock {
        #[arg(short, long, default_value = ".env")]
        env_file: String,
        #[arg(short, long, default_value = "./vault.key.pub")]
        recipient: String,
    },
    /// Decrypt the vault into a plaintext env file
    Unlock {
        #[arg(short, long, default_value = ".env.decrypted")]
        output: String,
        #[arg(short, long, default_value = "./vault.key")]
        identity: String,
    },
    /// List keys stored in the vault
    List,
    /// Run a command with decrypted secrets injected into environment
    Run {
        #[arg(short, long, default_value = "./vault.age")]
        vault: String,
        #[arg(short, long, default_value = "./vault.key")]
        identity: String,
        /// The command and its arguments
        #[arg(trailing_var_arg = true, required = true)]
        command: Vec<String>,
    },
}
