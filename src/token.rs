use crate::intern::*;

use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Error,

    #[regex(r"\$[_a-zA-Z0-9]+", cb_name)]
    Name(IString),
    // TODO: URL regex
    Url(IString),
}

fn cb_name(l: &mut Lexer<Token>) -> IString {
    l.slice()[1..].id()
}
