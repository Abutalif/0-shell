use crate::write_stdout;
pub struct Echo;

impl Echo {
    pub fn run(input: &str) {
        let _ = write_stdout(input);
        // do something with error
    }
}