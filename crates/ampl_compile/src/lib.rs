mod lexer;
mod parser;

use ampl_ast::ExprKind;
use parser::{ParseError, Parser};

pub fn compile(source: &str) -> Result<ExprKind, ParseError> {
    let mut parser = Parser::new(source);

    parser.parse()
}
