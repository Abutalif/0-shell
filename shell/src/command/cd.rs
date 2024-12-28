use std::{env, io};


pub struct Cd {
    dest: String,
}

impl Cd {
    pub fn new(d: &str) -> Self {
        Cd { dest: d.to_owned() }
    }

    pub fn run(&self) -> io::Result<()> {
        env::set_current_dir(self.dest.to_owned())
    }
}