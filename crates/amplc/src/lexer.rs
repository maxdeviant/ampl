use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[regex("[a-zA-Z]+")]
    Symbol,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}
