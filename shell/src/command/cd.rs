use std::{env, io};


pub struct Cd {
    dest: String,
}

impl Cd {
    pub fn new(dest: String) -> Self {
        Cd {
            dest
        }
    }

    pub fn run(&self) -> io::Result<()> {
        env::set_current_dir(self.dest.to_owned())
    }
}