pub mod keygen;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// vaultkeeper — encrypt and manage local .env files with age encryption
#[derive(Parser, Debug)]
#[command(name = "vaultkeeper", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new age key pair for encryption/decryption
    Keygen {
        /// Optional path to write the private key file (default: ./vaultkeeper.key)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
