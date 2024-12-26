mod echo;

use std::io;

use echo::Echo;

pub enum Command {
    Echo(Echo),
    Cd,
    Ls,
    Pwd,
    Cat,
    Cp,
    Rm,
    Mv,
    Mkdir,
    Clear,
    Exit,
}

impl Command {
    pub fn run(self) {
        match self {
            Self::Echo(echo) => echo.run(),
            _ => (),
        };
    }
}

impl TryFrom<&str> for Command {
    type Error = io::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // let command_with_params = command.split_ascii_whitespace();
        let mut commands = line.split_ascii_whitespace();
        let command = commands.next().unwrap();

        let command = match command {
            "echo" => Self::Echo(Echo::new(commands.collect::<Vec<_>>().join(" "))),
            "cd" => Self::Cd,
            "ls" => Self::Ls,
            "pwd" => Self::Pwd,
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