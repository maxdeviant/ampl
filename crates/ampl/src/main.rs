use ampl_compile::compile;
use ampl_eval::eval;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {}

fn main() {
    let args = Args::parse();

    let json = r#"{"foo":{"bar":{"baz":"Hello from Ampl"}}}"#;

    let compile_result = compile("(. foo bar baz)").expect("compile error");

    let eval_result = eval(compile_result, json).expect("evaluation error");

    println!("{}", eval_result)
}
