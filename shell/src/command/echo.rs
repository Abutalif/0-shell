pub struct Echo {
    flags: Vec<String>,
}

impl Echo {
    pub fn new(flags: Vec<String>) -> Self {
        Echo {
            flags
        }
    }
    pub fn run(&self) -> String {
        format!("{}\n", self.flags.join(" "))
    }
}