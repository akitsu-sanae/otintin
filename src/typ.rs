use env::Env;
use expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Arrow(Box<Type>, Box<Type>),
}

use std::fmt::{self, Formatter, Display};
impl Display for Type {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::Type::*;
        match *self {
            Int => write!(format, "Int"),
            Arrow(box ref ty1, box ref ty2) => write!(format, "{} -> ({})", ty1, ty2),
        }
    }
}

pub fn check(e: &Expr) -> Result<Type, String> {
    Ok(Type::Int) // TODO:
}

