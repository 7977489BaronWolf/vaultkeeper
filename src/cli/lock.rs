use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Args;

use crate::crypto::encrypt::encrypt_file;

/// Encrypt (lock) an env file using a recipient's age public key
#[derive(Args, Debug)]
pub struct LockArgs {
    /// Path to the plaintext env file to encrypt
    #[arg(short, long, default_value = ".env")]
    pub input: PathBuf,

    /// Path for the encrypted output file
    #[arg(short, long, default_value = ".env.age")]
    pub output: PathBuf,

    /// Age public key (recipient) to encrypt for
    #[arg(short, long)]
    pub recipient: String,
}

pub fn run(args: LockArgs) -> Result<()> {
    if !args.input.exists() {
        anyhow::bail!("Input file '{}' does not exist", args.input.display());
    }

    encrypt_file(&args.input, &args.output, &args.recipient)
        .with_context(|| format!("Failed to encrypt '{}'", args.input.display()))?;

    println!(
        "🔒 Locked '{}' → '{}'",
        args.input.display(),
        args.output.display()
    );
    Ok(())
}
