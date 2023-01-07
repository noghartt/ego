use std::{iter::Peekable, str::Chars};

use super::{location::{Spanned, Point}, token::Token};

pub struct Lexer<'a> {
  input: &'a str,
  peekable: Peekable<Chars<'a>>,
  start_pos: Point,
  current_pos: Point,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
        input,
        peekable: input.chars().peekable(),
        start_pos: Default::default(),
        current_pos: Default::default(),
        }
    }

    pub fn single_token(&mut self, token: Token<'a>) -> Spanned<Token<'a>> {
        if let Some(_) = self.peekable.next() {
            self.current_pos += 1;
        }

        self.make_token(token)
    }

    pub fn make_token(&self, token: Token<'a>) -> Spanned<Token<'a>> {
        Spanned::new(
            self.start_pos.clone(),
            self.current_pos.clone(),
            token
        )
    }

    pub fn accumulate_while(&mut self, pred: fn(&char) -> bool) -> &'a str {
        while self.peekable.peek().map(pred).unwrap_or_default() {
            self.peekable.next();
            self.current_pos += 1;
        }

        &self.input[self.start_pos..self.current_pos]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.start_pos = self.current_pos.clone();
        let char = self.peekable.peek()?;

        let result = match char {
            c if is_identifier(c) => {
                match self.accumulate_while(is_identifier) {
                    "let" => self.make_token(Token::Let),
                    "in" => self.make_token(Token::In),
                    id => self.make_token(Token::Id(id)),
                }
            }
            _ => self.single_token(Token::Error),
        };

        Some(result)
    }
}

fn is_identifier(char: &char) -> bool {
    matches!(char, '_' | 'a'..='z' | 'A'..='Z')
}

#[cfg(test)]
mod tests {
    use super::Spanned;
    use super::Token;
    use super::Lexer;

  #[test]
  fn should_create_a_new_lexer() {
    let input: &str = "x = \"10\"";
    let lexer = Lexer::new(input);

    assert_eq!(lexer.input, input, "testing if lexer input has been right assigned");
  }

  #[test]
  fn should_lex_let_identifier() {
    let input = "let";
    let mut lexer = Lexer::new(input);

    let next_token = lexer.next().unwrap();

    assert_eq!(next_token.data, Token::Let);
    assert_eq!(next_token.span, 0..3);
  }
}
