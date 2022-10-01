mod lexer;
mod parser;

use crate::parser::Parser;

pub enum Expr {
    Symbol,
}

fn main() {
    let mut parser = Parser::new("(foo)");

    let result = parser.parse();

    println!("{:?}", result);
}
