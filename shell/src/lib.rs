pub mod command;
pub mod syscall;

use std::{env, io::{self}, path::PathBuf};

pub struct Shell; // so far just a unit type.

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn _execute(_line: &str) -> Result<String, io::Error> {
        todo!()
    }
}

pub fn read_stdin() -> io::Result<String> {
    let mut input = String::new();
    let _bytes = io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn write_stdout(msg: &str) -> io::Result<()> {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    let res = unsafe {syscall::write(syscall::STDOUT, msg_ptr, len)};
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    Ok(())
}

pub fn get_cwd()->io::Result<PathBuf> {
    env::current_dir()
}