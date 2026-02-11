mod cli;
mod init;
mod watcher;
mod log;
mod state;   // <-- only add this when state.rs is ready

fn main() -> anyhow::Result<()> {
    let command = cli::parse();
    cli::dispatch(command)
}
