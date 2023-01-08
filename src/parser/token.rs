#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Let,
    In,

    Id(&'a str),
    Int(usize),
    Str(String),

    LPar,          // (
    RPar,          // )
    Equal,         // =

    Plus,          // +
    Star,          // *

    Error,
}
