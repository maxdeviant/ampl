use logos::Logos;

/// The representation of an Ampl operator.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorRepr {
    /// The operator is represented using ASCII.
    Ascii,

    /// The operator is represented using Unicode.
    Unicode,
}

/// An Ampl token.
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("λ", operator_repr)]
    #[token(r"\", operator_repr)]
    Lambda(OperatorRepr),

    #[regex("[a-zA-Z]+")]
    Symbol,

    #[regex(r"[ \n]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

fn operator_repr(lexer: &mut logos::Lexer<Token>) -> OperatorRepr {
    match lexer.slice().is_ascii() {
        true => OperatorRepr::Ascii,
        false => OperatorRepr::Unicode,
    }
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lexer.next()?;
        let lexeme = self.lexer.slice();

        Some((token, lexeme))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, token: Token) {
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(token));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lex_left_paren() {
        check("(", Token::LeftParen)
    }

    #[test]
    fn lex_right_paren() {
        check(")", Token::RightParen)
    }

    #[test]
    fn lex_lambda() {
        check("λ", Token::Lambda(OperatorRepr::Unicode))
    }

    #[test]
    fn lex_ascii_lambda() {
        check(r"\", Token::Lambda(OperatorRepr::Ascii))
    }
}
