mod cli;
mod init;
mod watcher;

fn main() -> anyhow::Result<()> {
    let command = cli::parse();
    cli::dispatch(command)
}
