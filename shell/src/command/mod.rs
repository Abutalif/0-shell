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
    Exit,
}

impl TryFrom<&str> for Command {
    type Error = io::Error;

    fn try_from(_command: &str) -> Result<Self, Self::Error> {
        // let command_with_params = command.split_ascii_whitespace();
        todo!()
    }
}