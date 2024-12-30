use std::{io, path::PathBuf, str::FromStr};

pub struct Ls {
    paths: Vec<PathBuf>,
    flags: Vec<Flag>,
}

enum Flag {
    List,
    All,
    Full
}

impl Ls {    
    pub fn run(&self) {

    }
}

// TODO: do logic. return error if flag is not parsable.
fn parse_flag(_input: &str) -> Flag {
    Flag::All
}

fn parse_dir(input: &str) -> PathBuf {
    PathBuf::from(input)
}

impl FromStr for Ls {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // for item in input {
        //     if item.starts_with("-") {
        //         flags.push(parse_flag(item)); 
        //     } else {
        //         paths.push(parse_dir(item));
        //     }
        // }
        Ok(Ls{
            paths: Vec::new(),
            flags: Vec::new(),
        })
    }
}



// here is what happens when we call new()
// we iterate over entries
// if entry prefixes with "-", we try to turn it into Flag.
// if cannot be parsed into flag - return error (unsupported option),
// if can be, parse and then push into vec
// if it does not start with -, its a candidate for a search
// save it into vec 