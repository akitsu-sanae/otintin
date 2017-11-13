
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add, Sub, Mult, Div
}

use std::fmt::{self, Formatter, Display};

impl Display for BinOp {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "{}", match *self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mult => "*",
            BinOp::Div => "/",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Func(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Int(n) => write!(format, "{}", n),
            Var(ref x) => write!(format, "{}", x),
            BinOp(ref op, box ref lhs, box ref rhs) =>
                write!(format, "({} {} {})", lhs, op, rhs),
            Func(ref x, box ref e) =>
                write!(format, "(func {} => {})", x, e),
            App(box ref lhs, box ref rhs) =>
                write!(format, "({}@{})", lhs, rhs),
            Let(ref x, box ref e1, box ref e2) =>
                write!(format, "let {} = {} in {}", x, e1, e2),
        }
    }
}

