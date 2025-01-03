use std::{env, io, str::FromStr};


pub struct Cd {
    dest: String,
}

impl Cd {
    pub fn run(&self) -> io::Result<()> {
        env::set_current_dir(self.dest.to_owned())
    }
}

impl FromStr for Cd {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cd { dest: s.to_owned() })
    }
}