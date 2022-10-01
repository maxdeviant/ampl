/// The kind of an Ampl expression.
#[derive(Debug, Clone)]
pub enum ExprKind {
    Dot(Vec<ExprKind>),
    Symbol(String),
    List(Vec<ExprKind>),
}
