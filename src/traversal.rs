use crate::cli::Args;
use anyhow::{Context, Result};
use std::fs;

pub fn run(args: &Args) -> Result<()> {
    let _meta = fs::symlink_metadata(&args.root)
        .with_context(|| format!("cannot access {}", args.root.display()))?;

    Ok(())
}
