use crate::intern::*;
use crate::object::*;

pub enum Value {
    Url(IString),
    Object(Object),
}
