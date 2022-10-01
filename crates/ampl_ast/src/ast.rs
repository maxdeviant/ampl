/// The kind of an Ampl expression.
#[derive(Debug, Clone)]
pub enum ExprKind {
    Symbol(String),
    List(Vec<ExprKind>),
}
