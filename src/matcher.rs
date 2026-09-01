use crate::cli::{Args, Kind};
use std::{fs::Metadata, path::Path};

pub fn matches(args: &Args, path: &Path, meta: &Metadata) -> bool {
    if let Some(kind) = &args.kind {
        match kind {
            Kind::File => {
                if !meta.is_file() {
                    return false;
                }
            }
            Kind::Dir => {
                if !meta.is_dir() {
                    return false;
                }
            }
            Kind::Symlink => {
                if !meta.is_symlink() {
                    return false;
                }
            }
        }
    }

    if let Some(filename) = path.file_name() {
        if let Some(name) = &args.name {
            if name.as_str() != filename {
                return false;
            }
        }
    } else {
        return false;
    }
    true
}
