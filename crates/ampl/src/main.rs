use std::process::exit;

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

    let compile_result = match compile(&args.expr) {
        Ok(expr) => expr,
        Err(err) => {
            println!("{}", err.to_string());
            exit(1)
        }
    };

    let eval_result = match eval(&compile_result, &args.json) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    };

    println!("{}", eval_result)
}

#[cfg(test)]
mod tests {
    use ampl_compile::compile;
    use ampl_eval::eval;
    use serde_json::json;

    fn check(source: &str, input: &str, expected: &str) {
        let compile_result = compile(source).expect("compile error");

        let eval_result = eval(&compile_result, input).expect("evaluation error");

        assert_eq!(&eval_result, expected)
    }

    #[test]
    fn basic_dot_expr() {
        check(
            "(. two)",
            &json!({
                "one": 1,
                "two": 2
            })
            .to_string(),
            "2",
        )
    }

    #[test]
    fn nested_dot_expr() {
        check(
            "(. one two three four)",
            &json!({
                "one": {
                    "two": {
                        "three": {
                            "four": 4
                        }
                    }
                },
                "two": 2
            })
            .to_string(),
            "4",
        )
    }

    #[test]
    fn equality_expr_is_true() {
        check(
            "(= (. two) 2)",
            &json!({
                "one": {
                    "two": {
                        "three": {
                            "four": 4
                        }
                    }
                },
                "two": 2
            })
            .to_string(),
            "true",
        )
    }

    #[test]
    fn equality_expr_is_false() {
        check(
            "(= (. one two) 2)",
            &json!({
                "one": {
                    "two": {
                        "three": {
                            "four": 4
                        }
                    }
                },
                "two": 2
            })
            .to_string(),
            "false",
        )
    }
}
