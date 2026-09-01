use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Simple find utility written in Rust.
#[derive(Debug, Parser)]
#[command(name = "riff", version, about)]
pub struct Args {
    #[arg(default_value = ".")]
    pub root: PathBuf,

    #[arg(short = 'n', long)]
    pub name: Option<String>,

    #[arg(short = 't', long = "type", value_enum)]
    pub kind: Option<Kind>,

    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum Kind {
    #[value(alias = "f")]
    File,
    #[value(alias = "d")]
    Dir,
    #[value(alias = "l")]
    Symlink,
}
