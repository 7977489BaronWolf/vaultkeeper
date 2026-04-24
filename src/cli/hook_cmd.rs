use clap::{Args, Subcommand};

/// Manage lifecycle hooks for vault events
#[derive(Debug, Args)]
pub struct HookArgs {
    #[command(subcommand)]
    pub subcommand: HookSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum HookSubcommand {
    /// Add a hook command for a lifecycle event
    Add {
        /// Event name (e.g. post-set, pre-lock, post-rotate)
        #[arg(value_name = "EVENT")]
        event: String,
        /// Shell command to execute
        #[arg(value_name = "COMMAND")]
        command: String,
    },
    /// Remove a hook command for a lifecycle event
    Remove {
        /// Event name
        #[arg(value_name = "EVENT")]
        event: String,
        /// Shell command to remove
        #[arg(value_name = "COMMAND")]
        command: String,
    },
    /// List all configured hooks
    List,
}

pub fn dispatch_hook(args: &HookArgs, config: &crate::config::Config) -> anyhow::Result<()> {
    match &args.subcommand {
        HookSubcommand::Add { event, command } => {
            crate::cli::hook::handle_hook_add(config, event, command)
        }
        HookSubcommand::Remove { event, command } => {
            crate::cli::hook::handle_hook_remove(config, event, command)
        }
        HookSubcommand::List => crate::cli::hook::handle_hook_list(config),
    }
}
