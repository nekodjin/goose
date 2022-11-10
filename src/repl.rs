use std::io;
use std::io::Write as _;

use crate::context::*;
use crate::intern::*;

pub async fn repl() {
    print_hello();

    // if current line is a continuation of the previous one
    let mut continuation = false;
    // the context for the REPL
    let mut context = Context::new();

    loop {
        if !continuation {
            print!("%- ");
        }
        else {
            print!("%: ");
        }

        io::stdout().flush().expect("err flushing stdout");

        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("err reading stdin");

        // End Of Input (blank line will contain a newline)
        if buffer == "" {
            break;
        }

        let line = buffer.trim().id();

        if line.starts_with(":") {
            let cmd = &line[1..];

            match execute_cmd(cmd) {
                CmdResult::Exit => return,
                CmdResult::Success => (),
                CmdResult::Unrecognized => {
                    println!("Unrecognized command: {cmd}");
                }
            }

            continue;
        }

        todo!();
    }

    print_goodbye();
}

fn print_hello() {
    println!("Welcome to Goose!");
}

fn print_goodbye() {
    println!("Bye-bye!");
}

enum CmdResult {
    Exit,
    Success,
    Unrecognized,
}

fn execute_cmd(cmd: &str) -> CmdResult {
    match cmd {
        "q" | "quit" => CmdResult::Exit,
        _ => CmdResult::Unrecognized,
    }
}
