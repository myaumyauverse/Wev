use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "statemesh")]
#[command(about = "State-based file evolution tracker", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize StateMesh in a folder
    Init {
        path: String,
    },

    /// Watch a folder for changes
    Watch {
        path: String,
    },
}

pub fn parse() -> Commands {
    Cli::parse().command
}

pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::Init { path } => {
            println!("Initializing StateMesh in {}", path);
            Ok(())
        }
        Commands::Watch { path } => {
            println!("Watching folder {}", path);
            Ok(())
        }
    }
}
