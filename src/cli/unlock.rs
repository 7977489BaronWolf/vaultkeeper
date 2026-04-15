use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Args;

use crate::crypto::decrypt::decrypt_file;

/// Decrypt (unlock) an encrypted env file using an age identity key
#[derive(Args, Debug)]
pub struct UnlockArgs {
    /// Path to the encrypted .age file
    #[arg(short, long, default_value = ".env.age")]
    pub input: PathBuf,

    /// Path for the decrypted output file
    #[arg(short, long, default_value = ".env")]
    pub output: PathBuf,

    /// Path to the age identity (private key) file
    #[arg(short = 'k', long, default_value = "key.txt")]
    pub identity: PathBuf,
}

pub fn run(args: UnlockArgs) -> Result<()> {
    if !args.input.exists() {
        anyhow::bail!("Encrypted file '{}' does not exist", args.input.display());
    }

    if !args.identity.exists() {
        anyhow::bail!(
            "Identity file '{}' does not exist. Run `vaultkeeper keygen` to create one.",
            args.identity.display()
        );
    }

    // Warn the user if the output file already exists, so they are aware it will be overwritten
    if args.output.exists() {
        eprintln!(
            "⚠️  Warning: output file '{}' already exists and will be overwritten.",
            args.output.display()
        );
    }

    decrypt_file(&args.input, &args.output, &args.identity)
        .with_context(|| format!("Failed to decrypt '{}'", args.input.display()))?;

    println!(
        "🔓 Unlocked '{}' → '{}'",
        args.input.display(),
        args.output.display()
    );
    Ok(())
}
