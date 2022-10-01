use ampl_ast::ExprKind;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("failed to parse JSON")]
    InvalidJson(#[from] serde_json::Error),

    #[error("invalid expression")]
    InvalidExpr,
}

pub fn eval(expr: ExprKind, input: &str) -> Result<String, EvalError> {
    let value: Value = serde_json::from_str(input)?;

    match expr {
        ExprKind::Dot(exprs) => {
            let mut x = value;

            for expr in exprs {
                match expr {
                    ExprKind::Symbol(symbol) => match x {
                        Value::Object(object) => {
                            x = object
                                .get(&symbol)
                                .cloned()
                                .expect(&format!("no value found at '{}'", symbol))
                        }
                        _ => Err(EvalError::InvalidExpr)?,
                    },
                    _ => Err(EvalError::InvalidExpr)?,
                }
            }

            Ok(match x {
                Value::String(value) => value,
                value => value.to_string(),
            })
        }
        ExprKind::Symbol(symbol) => Ok(symbol),
        ExprKind::List(_) => Err(EvalError::InvalidExpr),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_works() {
        let json = json!({
            "foo": {
                "bar": {
                    "baz": "Hello, world!"
                }
            }
        });

        let expr = ExprKind::Dot(vec![
            ExprKind::Symbol("foo".to_string()),
            ExprKind::Symbol("bar".to_string()),
            ExprKind::Symbol("baz".to_string()),
        ]);

        let eval_result = eval(expr, &json.to_string()).unwrap();

        assert_eq!(eval_result, "Hello, world!")
    }
}
