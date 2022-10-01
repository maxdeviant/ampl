mod lexer;
mod parser;

use crate::parser::Parser;

pub enum Expr {
    Symbol,
}

fn main() {
    let mut parser = Parser::new("(. foo bar baz)");

    let result = parser.parse();

    match result {
        Ok(expr) => println!("{:?}", expr),
        Err(err) => println!("{}", err.to_string()),
    }
}
