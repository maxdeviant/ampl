use ampl_compile::compile;

fn main() {
    let result = compile("(. foo bar baz)");

    match result {
        Ok(expr) => println!("{:?}", expr),
        Err(err) => println!("{}", err.to_string()),
    }
}
