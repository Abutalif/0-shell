use std::{io, str::FromStr};
pub struct Echo {
    input: String
}

impl Echo {
    pub fn new(s: &str) -> Self {
        Self {input: s.to_owned()}
    }
}