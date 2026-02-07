mod cli;
mod init;
mod watcher;
mod log;


fn main() -> anyhow::Result<()> {
    let command = cli::parse();
    cli::dispatch(command)
}
