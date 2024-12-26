use shell::{read_stdin, write_stdout, Shell};
use shell::command::Command;

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
        if let Ok(command) = Command::try_from(input.as_str()) {
            if let Some(output) = command.run() {
                write_stdout(&output).expect("Oops, when printing output");
            }
        }
    }
}
