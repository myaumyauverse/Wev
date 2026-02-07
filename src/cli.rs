use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "wev")]
#[command(about = "Write Event Versioning")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize Wev in a folder
    Init {
        path: String,
    },

    /// Watch a folder for file changes
    Watch {
        path: String,
    },

    /// Show StateMesh event history
    Log,
}

pub fn parse() -> Commands {
    Cli::parse().command
}

pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::Init { path } => {
            crate::init::init_workspace(&path)?;
        }

        Commands::Watch { path } => {
            crate::watcher::watch_folder(&path)?;
        }

        Commands::Log => {
            crate::log::print_log()?;
        }
    }

    Ok(())
}
