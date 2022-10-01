use ampl_compile::compile;
use ampl_eval::eval;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    expr: String,
    json: String,
}

fn main() {
    let args = Args::parse();

    let compile_result = compile(&args.expr).expect("compile error");

    let eval_result = eval(compile_result, &args.json).expect("evaluation error");

    println!("{}", eval_result)
}
