use std::{io, str::FromStr};

pub struct Cp {
    args: String,
}

impl FromStr for Cp {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cp { args: s.to_owned() })
    }
}
