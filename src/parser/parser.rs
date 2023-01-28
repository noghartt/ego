use std::iter::Peekable;

use super::{
    lexer::Lexer,
    location::Spanned,
    token::Token,
    error::SyntaxError,
    tree::{Expr, ExprKind, LetNode, BinaryNode, Operation, UnaryNode}
};

pub type Result<'input, T, E = SyntaxError<'input>> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse_let(&mut self) -> Result<'a, Box<Expr>> {
        let start = self.eat(Token::Let)?;
        let name = self.parse_id()?;
        self.eat(Token::Equal)?;
        let value = self.parse_expr()?;
        self.eat(Token::In)?;
        let next = self.parse_expr()?;
        Ok(Box::new(Expr {
            span: start.span.start..next.span.end,
            data: ExprKind::Let(LetNode {
                name: name.data.to_string(),
                value,
                next,
            }),
        }))
    }

    pub fn parse_expr(&mut self) -> Result<'a, Box<Expr>> {
        let Some(token) = self.lexer.peek() else {
            return Err(SyntaxError::UnexpectedEOF)
        };

        match token.data {
            Token::Let => self.parse_let(),
            _ => self.parse_add(),
        }
    }

    pub fn parse_add(&mut self) -> Result<'a, Box<Expr>> {
        let mut left = self.parse_mult()?;
        while self.check(Token::Plus) || self.check(Token::Minus) {
            let Some(token) = self.next() else {
                return Err(SyntaxError::UnexpectedEOF)
            };
            let right = self.parse_mult()?;
            left = Box::new(Expr {
                span: left.span.start..right.span.end,
                data: ExprKind::Binary(BinaryNode {
                    operation: if token.data == Token::Plus { Operation::Add } else { Operation::Minus },
                    left,
                    right,
                }),
            });
        }
        Ok(left)
    }

    pub fn parse_mult(&mut self) -> Result<'a, Box<Expr>> {
        let mut left = self.parse_atom()?;
        while self.check(Token::Star) || self.check(Token::Slash) {
            let Some(token) = self.next() else {
                return Err(SyntaxError::UnexpectedEOF)
            };
            let right = self.parse_atom()?;
            left = Box::new(Expr {
                span: left.span.start..right.span.end,
                data: ExprKind::Binary(BinaryNode {
                    operation: if token.data == Token::Star { Operation::Mult } else { Operation::Slash },
                    left,
                    right,
                }),
            });
        }
        Ok(left)
    }

    pub fn parse_parens(&mut self) -> Result<'a, Box<Expr>> {
        self.eat(Token::LPar)?;
        let expr = self.parse_expr()?;
        self.eat(Token::RPar)?;
        Ok(expr)
    }

    pub fn parse_atom(&mut self) -> Result<'a, Box<Expr>> {
        let Some(token) = self.lexer.peek() else {
            return Err(SyntaxError::UnexpectedEOF)
        };

        match token.data {
            Token::Int(_) => self.parse_int(),
            Token::Id(_) => self.parse_ident(),
            Token::LPar => self.parse_parens(),
            Token::Minus => self.parse_minus(),
            _ => Err(SyntaxError::UnexpectedToken(token.to_owned())),
        }
    }

    pub fn parse_minus(&mut self) -> Result<'a, Box<Expr>> {
        let op = self.eat(Token::Minus)?;
        let expr = self.parse_expr()?;
        Ok(Box::new(Expr {
            span: op.span.start..expr.span.end,
            data: ExprKind::Unary(UnaryNode {
                operation: Operation::Minus,
                expr,
            }),
        }))
    }

    pub fn parse_ident(&mut self) -> Result<'a, Box<Expr>> {
        let name = self.parse_id()?;
        Ok(Box::new(Expr {
            span: name.span.clone(),
            data: ExprKind::Ident(name.data.to_string()),
        }))
    }

    pub fn parse_int(&mut self) -> Result<'a, Box<Expr>> {
        let int = self.eat_match(&|t| {
            match t {
                Token::Int(x) => Some(x),
                _ => None,
            }
        })?;

        Ok(Box::new(Expr {
            span: int.span.clone(),
            data: ExprKind::Int(int.data),
        }))
    }

    pub fn parse_id(&mut self) -> Result<'a, Spanned<&'a str>> {
        self.eat_match(&|t| {
            match t {
                Token::Id(x) => Some(x),
                _ => None,
            }
        })
    }

    pub fn eat(&mut self, token: Token) -> Result<'a, Spanned<Token<'a>>> {
        self.eat_match(&|tkn| {
            if tkn == token {
                Some(tkn.clone())
            } else {
                None
            }
        })
    }

    pub fn check(&mut self, token: Token) -> bool {
        self.lexer
            .peek()
            .and_then(|x| Some(x.data == token))
            .unwrap_or(false)
    }

    pub fn next(&mut self) -> Option<Spanned<Token<'a>>> {
        self.lexer.next()
    }

    pub fn eat_match<T>(&mut self, fun: &dyn Fn(Token<'a>) -> Option<T>) -> Result<'a, Spanned<T>>  {
        let Some(token) = self.lexer.peek() else {
            return Err(SyntaxError::UnexpectedEOF)
        };

        if let Some(res) = fun(token.data.clone()) {
            let token = self.lexer.next().unwrap();
            Ok(Spanned {
                data: res,
                span: token.span,
            })
        } else {
            Err(SyntaxError::UnexpectedToken(token.to_owned()))
        }
    }
}
