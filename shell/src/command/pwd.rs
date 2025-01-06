use std::{io, path::PathBuf, str::FromStr};

use crate::current_dir;

pub struct Pwd {
    flags: String,
}

impl Pwd {
    pub fn run(&self, cwd: &PathBuf) -> String {
        cwd.display().to_string() + "\n"
    }
}

impl FromStr for Pwd {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pwd {
            flags: s.to_owned(),
        })
    }
}
