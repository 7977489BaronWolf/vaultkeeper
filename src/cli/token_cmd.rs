use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub command: TokenCommand,
}

#[derive(Debug, Subcommand)]
pub enum TokenCommand {
    /// Create a new access token
    Create {
        /// Human-readable label for the token
        #[arg(short, long)]
        label: String,

        /// Comma-separated scopes (e.g. read,write or *)
        #[arg(short, long, default_value = "read")]
        scopes: String,

        /// Optional TTL in seconds
        #[arg(short, long)]
        ttl: Option<u64>,
    },

    /// Revoke a token by ID
    Revoke {
        /// Token ID to revoke
        id: String,
    },

    /// List all active (non-expired) tokens
    List,

    /// Remove all expired tokens
    Purge,

    /// Show details of a specific token
    Inspect {
        /// Token ID to inspect
        id: String,
    },
}

pub fn handle_token(args: TokenArgs, store: &mut crate::store::token::TokenStore) -> anyhow::Result<()> {
    use super::token::*;
    use TokenCommand::*;
    match args.command {
        Create { label, scopes, ttl } => {
            let scope_list: Vec<String> = scopes.split(',').map(|s| s.trim().to_string()).collect();
            cmd_token_create(store, &label, scope_list, ttl)?;
        }
        Revoke { id } => cmd_token_revoke(store, &id)?,
        List => cmd_token_list(store)?,
        Purge => cmd_token_purge(store)?,
        Inspect { id } => cmd_token_inspect(store, &id)?,
    }
    Ok(())
}
