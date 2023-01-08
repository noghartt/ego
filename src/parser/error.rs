use super::{location::Spanned, token::Token};

#[derive(Debug)]
pub enum SyntaxError<'a> {
    UnexpectedToken(Spanned<Token<'a>>),
    UnexpectedEOF,
}
