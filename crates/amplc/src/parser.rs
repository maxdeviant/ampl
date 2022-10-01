use std::iter::Peekable;

use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Expr {
    Symbol,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expr()
    }

    fn peek(&mut self) -> Option<Token> {
        self.lexer.peek().map(|(token, _)| *token)
    }

    fn check(&mut self, ty: Token) -> bool {
        match self.peek() {
            Some(token) => token == ty,
            None => false,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.lexer.next().map(|(token, _)| token)
    }

    fn consume(&mut self, ty: Token, message: &str) -> Result<Token, String> {
        if self.check(ty) {
            self.advance().ok_or("Empty".to_string())
        } else {
            Err(message.to_string())
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.consume(Token::LeftParen, "Expected '('")?;
        self.consume(Token::Symbol, "Expected symbol")?;
        self.consume(Token::RightParen, "Expected ')'")?;

        Ok(Expr::Symbol)
    }
}
