use std::{io, str::FromStr};

use crate::current_dir;

pub struct Pwd {
    flags: String,
}

impl Pwd {
    pub fn run(&self) -> String {
        format!("{}\n", current_dir().unwrap_or_default().display().to_string())
    }
}

impl FromStr for Pwd {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pwd{
            flags: s.to_owned(),
        })
    }
}