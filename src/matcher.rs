use crate::{
    cli::{Args, Kind},
    glob::Pattern,
};
use std::{ffi::OsStr, fs::Metadata, os::unix::ffi::OsStrExt};

pub struct Matcher {
    name: Option<Pattern>,
    kind: Option<Kind>,
}

impl Matcher {
    pub fn from_args(args: &Args) -> anyhow::Result<Matcher> {
        let Some(patt) = &args.name else {
            return Ok(Matcher {
                name: None,
                kind: args.kind,
            });
        };

        let pattern = Pattern::parse(patt.as_bytes());

        Ok(Matcher {
            name: Some(pattern),
            kind: args.kind,
        })
    }

    pub fn matches(&self, name: &OsStr, meta: &Metadata) -> bool {
        if let Some(kind) = self.kind {
            let kind_match = match kind {
                Kind::File => meta.is_file(),
                Kind::Dir => meta.is_dir(),
                Kind::Symlink => meta.is_symlink(),
            };
            if !kind_match {
                return false;
            }
        }

        match &self.name {
            Some(patt) => patt.matches(name.as_bytes()),
            None => true,
        }
    }
}
