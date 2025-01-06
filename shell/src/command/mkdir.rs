use std::{io, str::FromStr};

pub struct Mkdir {
    args: String,
}

impl FromStr for Mkdir {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mkdir { args: s.to_owned() })
    }
}
