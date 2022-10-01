pub mod token;

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
    type Item = (TokenKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next()?;
        let lexeme = self.lexer.slice();

        Some((token, lexeme))
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
}