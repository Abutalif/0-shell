pub mod command;
pub mod syscall;

use std::{env, io::{self}, path::PathBuf};

pub struct Shell {
    cwd: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            cwd: get_cwd().expect("Error: Could not get current working directory!"),
        }
    }

    pub fn show_cwd(&self) -> String {
        self.cwd.display().to_string().to_owned()
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

fn get_cwd()->io::Result<PathBuf> {
    env::current_dir()
}