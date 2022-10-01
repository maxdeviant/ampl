/// The representation of an Ampl operator.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperatorRepr {
    /// The operator is represented using ASCII.
    Ascii,

    /// The operator is represented using Unicode.
    Unicode,
}

/// The kind of an Ampl token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Dot,
    Lambda(OperatorRepr),
    Symbol,
    Whitespace,
    Error,
}
