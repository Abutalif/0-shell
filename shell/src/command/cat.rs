use std::{io, str::FromStr};

pub struct Cat {
    flags: String,
}

impl FromStr for Cat {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cat{ flags: s.to_owned()})
    }
}

