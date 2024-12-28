mod echo;
mod pwd;
mod cd;
mod ls;

use std::io;
use std::str::FromStr;

use echo::Echo;
use pwd::Pwd;
use cd::Cd;
use ls::Ls;

pub enum Command {
    Echo(Echo),
    Cd(Cd),
    Ls(Ls),
    Pwd(Pwd),
    Cat,
    Cp,
    Rm,
    Mv,
    Mkdir,
    Clear,
    Exit,
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, options) = s.split_once(" ").unwrap_or((s, ""));

        let command = match command {
            "echo" => Self::Echo(Echo::new(options)),
            "cd" => Self::Cd(Cd::new(options)),
            "ls" => Self::Ls(Ls::new(options)),
            "pwd" => Self::Pwd(Pwd::new()),
            "cat" => Self::Cat,
            "cp" => Self::Cp,
            "rm" => Self::Rm,
            "mv" => Self::Mv,
            "mkdir" => Self::Mkdir,
            "clear" => Self::Clear,
            "exit" => Self::Exit,
            unknown => Err(io::Error::new(io::ErrorKind::Other, format!("{unknown} command unknown")))?,
        };

        Ok(command)
    }
}