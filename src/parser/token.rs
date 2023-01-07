#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Let,
    In,

    Id(&'a str),
    Int(usize),

    LPar,          // (
    RPar,          // )
    Equal,         // =

    Plus,          // +
    Star,          // *

    Error,
}
