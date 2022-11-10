use atty::Stream;

pub mod context;
pub mod intern;
pub mod object;
pub mod repl;
pub mod token;
pub mod value;

#[tokio::main]
async fn main() {
    if atty::is(Stream::Stdin) && atty::is(Stream::Stdout) {
        repl::repl().await;
    }
    else {
        eprintln!("Non-interactive mode is currently unsupported.");
    }
}
