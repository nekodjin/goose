use crate::intern::*;

pub enum Token {
    Name(IString),
    // TODO: URL regex
    Url(IString),
}
