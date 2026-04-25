use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct LockKeyArgs {
    #[command(subcommand)]
    pub subcommand: LockKeySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum LockKeySubcommand {
    /// Lock a secret key to prevent modification
    Lock {
        /// The key to lock
        key: String,
        /// Optional reason for locking
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// Unlock a previously locked secret key
    Unlock {
        /// The key to unlock
        key: String,
    },
    /// List all currently locked keys
    List,
}

pub fn handle_lock_key_cmd(
    args: &LockKeyArgs,
    store: &mut crate::store::lock_state::LockStateStore,
) -> Result<(), String> {
    use LockKeySubcommand::*;
    match &args.subcommand {
        Lock { key, reason } => {
            crate::cli::lock_key::run_lock_key(store, key, reason.as_deref())
        }
        Unlock { key } => {
            crate::cli::lock_key::run_unlock_key(store, key)
        }
        List => {
            crate::cli::lock_key::run_list_locked(store);
            Ok(())
        }
    }
}
