mod echo;
mod pwd;
mod cd;

use std::io;

use echo::Echo;
use pwd::Pwd;
use cd::Cd;

pub enum Command {
    Echo(Echo),
    Cd(Cd),
    Ls,
    Pwd(Pwd),
    Cat,
    Cp,
    Rm,
    Mv,
    Mkdir,
    Clear,
    Exit,
}

impl Command {
    // TODO: handle errors coming from each command
    pub fn run(self)-> Option<String> {
        match self {
            Self::Echo(echo) => Some(echo.run()),
            Self::Pwd(pwd) => Some(pwd.run()),
            Self::Cd(cd) => {cd.run(); None}
            _ => None,
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = io::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut commands = line.split_ascii_whitespace();
        let command = commands.next().unwrap_or_default();

        let command = match command {
            "echo" => Self::Echo(Echo::new(commands.map(|s|s.into()).collect::<Vec<_>>())),
            "cd" => Self::Cd(Cd::new(commands.collect())),
            "ls" => Self::Ls,
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

fn _parse_line() {}