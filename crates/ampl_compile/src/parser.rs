use std::iter::Peekable;

use ampl_ast::token::{Token, TokenKind};
use ampl_ast::ExprKind;

use crate::lexer::Lexer;

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(Option<usize>, String),
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            Self::SyntaxError(position, reason) => match position {
                Some(position) => format!("Syntax error at position {}: {}", position + 1, reason),
                None => format!("Syntax error: {}", reason),
            },
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

    pub fn parse(self) -> Result<ExprKind, ParseError> {
        let tokens: Vec<Token> = self.lexer.collect();

        let (expr, _) = parse(&tokens)?;

        Ok(expr)
    }
}

fn parse<'a>(tokens: &'a [Token]) -> Result<(ExprKind, &'a [Token<'a>]), ParseError> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(ParseError::SyntaxError(Some(1), "No token".to_string()))?;

    match token.kind {
        TokenKind::LeftParen => parse_list(rest),
        TokenKind::RightParen => Err(ParseError::SyntaxError(None, "Unexpected ')'".to_string())),
        _ => Ok((ExprKind::Symbol(token.lexeme.to_string()), rest)),
    }
}

fn parse_list<'a>(tokens: &'a [Token]) -> Result<(ExprKind, &'a [Token<'a>]), ParseError> {
    let mut list: Vec<ExprKind> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(ParseError::SyntaxError(None, "Expected ')'".to_string()))?;
        if next_token.kind == TokenKind::RightParen {
            return Ok((ExprKind::List(list), rest));
        }

        let (expr, new_xs) = parse(&xs)?;
        list.push(expr);
        xs = new_xs;
    }
}