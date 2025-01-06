use std::process::exit;

use shell::command::Command;
use shell::{read_stdin, write_stdout, Shell};

fn main() {
    let mut shell = Shell::new();
    loop {
        let msg = format!("{}$ ", shell.show_cwd());
        write_stdout(&msg).expect("Oops, when printing cwd");
        let input = read_stdin().unwrap_or("".into());

        if input.trim() == "exit" {
            break;
        }

        shell.save_command(input.clone());
        let command = input.parse::<Command>();
        // What the holy Fuck is this nesting?!
        if let Ok(command) = command {
            match command.run(&mut shell) {
                Ok(value) => {
                    if let Some(output) = value {
                        write_stdout(&output).expect("Oops, when printing output")
                    }
                }
                Err(err) => {
                    write_stdout(&(err.to_string() + "\n")).expect("Cannot write into stdout")
                }
            }
        }
    }
}
