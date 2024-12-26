use crate::current_dir;

pub struct Pwd;

impl Pwd {
    pub fn new() -> Self {
        Pwd
    }

    pub fn run(&self) -> String {
        format!("{}\n", current_dir().unwrap_or_default().display().to_string())
    }
}