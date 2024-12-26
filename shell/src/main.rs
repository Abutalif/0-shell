use shell::{read_stdin, write_stdout, Shell};
use shell::command::Command;

fn main() {
    let shell = Shell::new();
    loop {
        let msg = format!("{}$ ", shell.show_cwd());
        write_stdout(&msg).expect("Oops, when printing cwd");
        let input = read_stdin().unwrap_or("".into());

        if input.trim() == "exit" {
            break;
        }

        if let Ok(command) = Command::try_from(input.as_str()) {
            command.run()
        }
    }
}
