use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(
 name = "wev",
version,
about = "Write Event Versioning",
)]
pub struct Cli {
 #[command(subcommand)]
pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
/// Initialize a project (creates .wev/)
Init {
/// Path to initialize
path: String,
 },

/// Watch a folder and record semantic file events
Watch {
/// Path to watch
path: String,
 },

/// Show event history
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
