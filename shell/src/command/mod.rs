mod cat;
mod cd;
mod clear;
mod cp;
mod echo;
mod exit;
mod ls;
mod mkdir;
mod mv;
mod pwd;
mod rm;

use std::io;
use std::str::FromStr;

use cat::Cat;
use cd::Cd;
use clear::Clear;
use cp::Cp;
use echo::Echo;
use exit::Exit;
use ls::Ls;
use mkdir::Mkdir;
use mv::Mv;
use pwd::Pwd;
use rm::Rm;

use crate::Shell;

pub enum Command {
    Echo(Echo),
    Cd(Cd),
    Ls(Ls),
    Pwd(Pwd),
    Cat(Cat),
    Cp(Cp),
    Rm(Rm),
    Mv(Mv),
    Mkdir(Mkdir),
    Clear(Clear),
    Exit(Exit),
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, args) = s.trim().split_once(" ").unwrap_or((s.trim(), ""));
        let command = match command {
            "echo" => Self::Echo(args.parse()?),
            "cd" => Self::Cd(args.parse()?),
            "ls" => Self::Ls(args.parse()?),
            "pwd" => Self::Pwd(args.parse()?),
            "cat" => Self::Cat(args.parse()?),
            "cp" => Self::Cp(args.parse()?),
            "rm" => Self::Rm(args.parse()?),
            "mv" => Self::Mv(args.parse()?),
            "mkdir" => Self::Mkdir(args.parse()?),
            "clear" => Self::Clear(args.parse()?),
            "exit" => Self::Exit(args.parse()?),
            unknown => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{unknown} command unknown"),
            ))?,
        };

        Ok(command)
    }
}

impl Command {
    pub fn run(self, shell: &mut Shell) -> io::Result<Option<String>> {
        match self {
            Command::Echo(echo) => echo.run().map(|output| Some(output)),
            Command::Pwd(pwd) => Ok(Some(pwd.run(&shell.cwd))),
            Command::Cd(cd) => cd.run(&mut shell.cwd).map(|_| None),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "This command is yet to implement.",
            )),
        }
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
