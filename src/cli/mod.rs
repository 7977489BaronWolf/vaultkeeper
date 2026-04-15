pub mod keygen;
pub mod lock;
pub mod unlock;
pub mod list;
pub mod run;
pub mod edit;

#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod edit_tests;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new age key pair
    Keygen {
        #[arg(short, long, default_value = "identity.key")]
        output: String,
    },
    /// Encrypt a .env file into a vault
    Lock {
        #[arg(short, long, default_value = ".env")]
        input: String,
        #[arg(short, long, default_value = ".env.vault")]
        output: String,
        #[arg(short, long, default_value = "identity.pub")]
        key: String,
    },
    /// Decrypt a vault back to a .env file
    Unlock {
        #[arg(short, long, default_value = ".env.vault")]
        input: String,
        #[arg(short, long, default_value = ".env")]
        output: String,
        #[arg(short, long, default_value = "identity.key")]
        key: String,
    },
    /// List keys stored in the vault
    List {
        #[arg(short, long, default_value = ".env.vault")]
        vault: String,
        #[arg(short, long, default_value = "identity.key")]
        key: String,
    },
    /// Run a command with decrypted env vars injected
    Run {
        #[arg(short, long, default_value = ".env.vault")]
        vault: String,
        #[arg(short, long, default_value = "identity.key")]
        key: String,
        /// Command and arguments to execute
        #[arg(trailing_var_arg = true, required = true)]
        cmd: Vec<String>,
    },
    /// Open vault contents in $EDITOR and re-encrypt on save
    Edit {
        #[arg(short, long, default_value = ".env.vault")]
        vault: String,
        #[arg(short, long, default_value = "identity.key")]
        key: String,
    },
}

pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::Keygen { output } => keygen::handle_keygen(&output),
        Commands::Lock { input, output, key } => lock::handle_lock(&input, &output, &key),
        Commands::Unlock { input, output, key } => unlock::handle_unlock(&input, &output, &key),
        Commands::List { vault, key } => list::handle_list(&vault, &key),
        Commands::Run { vault, key, cmd } => run::handle_run(&vault, &key, &cmd),
        Commands::Edit { vault, key } => edit::handle_edit(&vault, &key),
    }
}
