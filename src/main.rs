mod cli;

fn main() -> anyhow::Result<()> {
    let command = cli::parse();
    cli::dispatch(command)
}
