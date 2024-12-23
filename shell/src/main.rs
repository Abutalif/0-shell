use std::{fmt::format, io::{self, Write}};

use shell::{read_stdin, write_stdout};

fn main() {
    loop {
        write_stdout("$ ");
        io::stdout().flush().unwrap();

        let input = read_stdin();
        //

        if input.trim() == "exit" {
            break;
        }
        let naive_echo = format!("$ naive echo: {}", input);
        write_stdout(&naive_echo);
    }
}
