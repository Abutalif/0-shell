use std::{io, str::FromStr};

pub struct Mv {
    args:String,
}

impl FromStr for Mv {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mv{args:s.to_owned()})
    }
}