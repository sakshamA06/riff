use std::fs::{Metadata, read_dir, symlink_metadata};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::cli::Args;
use crate::matcher::Matcher;

pub fn run(args: &Args) -> Result<()> {
    let metadata = symlink_metadata(&args.root)
        .with_context(|| format!("cannot access {}", args.root.display()))?;

    let mut stack = vec![(args.root.clone(), 0, metadata)];

    let matcher = Matcher::from_args(args).context("Ill formed pattern")?;

    while let Some((path, depth, metadata)) = stack.pop() {
        let name = path.file_name().unwrap_or(path.as_os_str());
        if matcher.matches(name, &metadata) {
            println!("{}", path.display());
        }

        if metadata.is_dir() && args.max_depth.is_none_or(|max| depth < max) {
            push_children(&mut stack, &path, depth);
        }
    }

    Ok(())
}

// NOTE: Failure doesnt abort
fn push_children(stack: &mut Vec<(PathBuf, usize, Metadata)>, dir: &Path, depth: usize) {
    let entries = match read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("riff: {}: {e}", dir.display());
            return;
        }
    };

    let mut children = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("riff: {}: {e}", dir.display());
                continue;
            }
        };

        match entry.metadata() {
            Ok(metadata) => children.push((entry.path(), depth + 1, metadata)),
            Err(e) => eprintln!("riff: {}: {e}", entry.path().display()),
        }
    }

    children.reverse();

    for child in children {
        stack.push(child);
    }
}
