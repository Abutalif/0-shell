use std::{env, io, path::PathBuf, str::FromStr};

use super::Shell;

pub struct Cd {
    dest: PathBuf,
}

impl Cd {
    pub fn run(self, cwd: &mut PathBuf) -> io::Result<()> {
        env::set_current_dir(self.dest.to_owned())
    }
}

impl FromStr for Cd {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cd { dest: PathBuf::from(s) })
    }
}