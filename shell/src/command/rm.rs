use std::{io, str::FromStr};

pub struct Rm {
    args: String,
}

impl FromStr for Rm {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rm { args: s.to_owned() })
    }
}
