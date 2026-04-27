use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct SigningArgs {
    #[command(subcommand)]
    pub command: SigningCommand,
}

#[derive(Debug, Subcommand)]
pub enum SigningCommand {
    /// Sign a secret value with your identity
    Sign {
        /// The secret key to sign
        key: String,
        /// The plaintext value to sign
        value: String,
        /// Signer identity (e.g. username or key ID)
        #[arg(short, long)]
        signer: String,
    },
    /// Verify the signature of a secret value
    Verify {
        /// The secret key to verify
        key: String,
        /// The plaintext value to verify against the stored signature
        value: String,
    },
    /// Show signature metadata for a key
    Show {
        /// The secret key whose signature to display
        key: String,
    },
    /// List all signed keys
    List,
    /// Remove the signature for a key
    Unsign {
        /// The secret key to unsign
        key: String,
    },
}

pub fn dispatch(args: SigningArgs, signing_path: &std::path::Path) {
    use super::signing::*;
    let mut store = load_signing_store(signing_path);
    match args.command {
        SigningCommand::Sign { key, value, signer } => {
            handle_sign(&mut store, &key, &value, &signer);
            save_signing_store(&store, signing_path).expect("Failed to save signing store");
        }
        SigningCommand::Verify { key, value } => {
            handle_verify(&store, &key, &value);
        }
        SigningCommand::Show { key } => {
            handle_show_signature(&store, &key);
        }
        SigningCommand::List => {
            handle_list_signed(&store);
        }
        SigningCommand::Unsign { key } => {
            handle_unsign(&mut store, &key);
            save_signing_store(&store, signing_path).expect("Failed to save signing store");
        }
    }
}
