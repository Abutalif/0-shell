use shell::{get_cwd, read_stdin, write_stdout, Shell};

fn main() {
    let _shell = Shell::new();
    loop {
        let cwd = get_cwd().unwrap();
        let msg = format!("{}$ ", cwd.display());
        write_stdout(&msg).expect("Oops, when printing cwd");
        // io::stdout().flush().unwrap(); - I have no idea what it does.
        let input = read_stdin().unwrap_or("".into());

        if input.trim() == "exit" {
            break;
        }

        write_stdout(&input).expect("Oops, when printing result");
    }
}
