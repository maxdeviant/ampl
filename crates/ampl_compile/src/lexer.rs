pub mod token;

use ampl_ast::token::Token;
use logos::Logos;

use crate::lexer::token::TokenKind;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let lexeme = self.lexer.slice();
        let span = self.lexer.span();

        Some(Token {
            kind: kind.into(),
            lexeme,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use ampl_ast::token::OperatorRepr;

    use super::*;

    fn check(input: &str, token: TokenKind) {
        let mut lexer = TokenKind::lexer(input);

        assert_eq!(lexer.next(), Some(token));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lex_left_paren() {
        check("(", TokenKind::LeftParen)
    }

    #[test]
    fn lex_right_paren() {
        check(")", TokenKind::RightParen)
    }

    #[test]
    fn lex_dot() {
        check(".", TokenKind::Dot)
    }

    #[test]
    fn lex_lambda() {
        check("λ", TokenKind::Lambda(OperatorRepr::Unicode))
    }

    #[test]
    fn lex_ascii_lambda() {
        check(r"\", TokenKind::Lambda(OperatorRepr::Ascii))
    }

    #[test]
    fn lex_alphabetic_symbol() {
        check("apple", TokenKind::Symbol)
    }

    #[test]
    fn lex_numeric_symbol() {
        check("1234", TokenKind::Symbol)
    }

    #[test]
    fn lex_alphanumeric_symbol() {
        check("abc123", TokenKind::Symbol)
    }

    #[test]
    fn lex_snake_case_symbol() {
        check("first_name", TokenKind::Symbol)
    }

    #[test]
    fn lex_screaming_snake_case_symbol() {
        check("FIRST_NAME", TokenKind::Symbol)
    }

    #[test]
    fn lex_kebab_case_symbol() {
        check("first-name", TokenKind::Symbol)
    }

    #[test]
    fn lex_version_symbol() {
        check("v1.2.3", TokenKind::Symbol)
    }

    #[test]
    fn lex_urn_symbol() {
        check(
            "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
            TokenKind::Symbol,
        )
    }
}
