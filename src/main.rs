mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { output } => {
            cli::keygen::run(output.as_deref())?;
        }
    }

    Ok(())
}
