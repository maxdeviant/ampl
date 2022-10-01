mod lexer;

use logos::Logos;

use crate::lexer::Token;

fn main() {
    let lexer = Token::lexer("(hello world)");

    for token in lexer {
        println!("{:?}", token);
    }
}
