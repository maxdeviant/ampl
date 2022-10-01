use ampl_ast::token as ast;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token(".")]
    Dot,

    #[token("λ", operator_repr)]
    #[token(r"\", operator_repr)]
    Lambda(ast::OperatorRepr),

    #[regex(r"[\w\.:-]+")]
    Symbol,

    #[regex(r"[ \n]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

impl From<TokenKind> for ast::TokenKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::LeftParen => ast::TokenKind::LeftParen,
            TokenKind::RightParen => ast::TokenKind::RightParen,
            TokenKind::Dot => ast::TokenKind::Dot,
            TokenKind::Lambda(repr) => ast::TokenKind::Lambda(repr),
            TokenKind::Symbol => ast::TokenKind::Symbol,
            TokenKind::Whitespace => ast::TokenKind::Whitespace,
            TokenKind::Error => ast::TokenKind::Error,
        }
    }
}

fn operator_repr(lexer: &mut logos::Lexer<TokenKind>) -> ast::OperatorRepr {
    match lexer.slice().is_ascii() {
        true => ast::OperatorRepr::Ascii,
        false => ast::OperatorRepr::Unicode,
    }
}
