use std::{io, str::FromStr};
pub struct Echo {
    input: String
}

impl Echo {
    pub fn run() {
        
    }
}

impl FromStr for Echo {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Echo {input: s.to_owned()})
    }
}