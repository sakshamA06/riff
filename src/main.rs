use anyhow::Result;
use clap::Parser;
use riff::{cli::Args, traversal::run};

fn main() -> Result<()> {
    let args = Args::parse();

    run(&args)?;

    Ok(())
}
