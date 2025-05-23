use clap::Parser;
use mytodo::{run, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
