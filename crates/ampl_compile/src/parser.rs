use std::iter::Peekable;

use ampl_ast::ExprKind;

use crate::lexer::token::TokenKind;
use crate::lexer::Lexer;

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(String),
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            Self::SyntaxError(reason) => {
                format!("Syntax error: {}", reason)
            }
        }
    }
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

    pub fn parse(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_expr()
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.lexer.peek().map(|(token, _)| *token)
    }

    fn check(&mut self, ty: TokenKind) -> bool {
        match self.peek() {
            Some(token) => token == ty,
            None => false,
        }
    }

    fn advance(&mut self) -> Option<(TokenKind, &'a str)> {
        self.lexer.next()
    }

    fn consume(
        &mut self,
        ty: TokenKind,
        message: &str,
    ) -> Result<(TokenKind, &'a str), ParseError> {
        if self.check(ty) {
            self.advance()
                .ok_or(ParseError::SyntaxError("Empty".to_string()))
        } else {
            Err(ParseError::SyntaxError(message.to_string()))
        }
    }

    fn parse_dot_expr(&mut self) -> Result<ExprKind, ParseError> {
        self.consume(TokenKind::LeftParen, "Expected '('")?;
        self.consume(TokenKind::Dot, "Expected '.'")?;

        let mut operands = vec![];
        loop {
            match self.parse_expr() {
                Ok(expr) => operands.push(expr),
                Err(_) => {}
            }

            if self.peek() == Some(TokenKind::RightParen) {
                break;
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')'")?;

        Ok(ExprKind::Dot(operands))
    }

    fn parse_symbol_expr(&mut self) -> Result<ExprKind, ParseError> {
        let (_, lexeme) = self.consume(TokenKind::Symbol, "Expected symbol")?;

        Ok(ExprKind::Symbol(lexeme.to_string()))
    }

    fn parse_expr(&mut self) -> Result<ExprKind, ParseError> {
        self.parse_dot_expr().or_else(|_| self.parse_symbol_expr())
    }
}