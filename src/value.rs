use expr::Expr;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Func(String, Box<Expr>),
}

