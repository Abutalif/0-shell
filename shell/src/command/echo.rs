use crate::write_stdout;
pub struct Echo {
    input: String,
}

impl Echo {
    pub fn new(input: String) -> Self {
        Echo {
            input
        }
    }
    pub fn run(&self) {
        let _ = write_stdout(&format!("{}\n", self.input));
        // TODO: do something with error
    }
}