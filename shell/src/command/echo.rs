use std::{io, str::FromStr};
pub struct Echo {
    input: String,
    with_nl: bool,
}

impl Echo {
    pub fn run(self) -> io::Result<String> {
        let trailing = if self.with_nl {"\n"} else {""};
        Ok(self.input + trailing)
    }
}

impl FromStr for Echo {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (input, with_nl) = match s.trim_start().strip_prefix("-n") {
            Some(x) => (x.trim().to_owned(), false),
            None => (s.to_owned(), true)
        };
        Ok(Echo {input, with_nl})
    }
}

// not there yet. we need it in the future. it will wok
fn tokenize(input: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut token = String::new();
    
    for x in input.chars() {
        if x != ' ' {
            token.push(x);
        } else {
            res.push(token.clone());
            token.clear();
        }
    }
    
    if !token.is_empty() {
        res.push(token);
    }
    // hello there "friend"
    res
}