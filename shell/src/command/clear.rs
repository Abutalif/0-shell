use std::{io, str::FromStr};

pub struct Clear {
    args:String,
}

impl FromStr for Clear {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Clear{args:s.to_owned()})
    }
}