use std::{io, str::FromStr};

pub struct Exit {
    args: String,
}

impl FromStr for Exit {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Exit { args: s.to_owned() })
    }
}
