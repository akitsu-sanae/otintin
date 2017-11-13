use expr::{BinOp, Expr};
use env::Env;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Func(String, Box<Expr>),
}

macro_rules! eval_binop {
    ($lhs:expr, $rhs:expr, $env:expr, $f:expr, $msg:expr) => {
        match (eval($lhs, $env)?, eval($rhs, $env)?) {
            (Value::Int(lhs), Value::Int(rhs)) => {
                Ok(Value::Int($f(lhs, rhs)))
            },
            _ => Err(format!("can not {} {} and {}", $msg, $lhs, $rhs)),
        }
    };
}

pub fn eval(e: &Expr, env: &Env<Value>) -> Result<Value, String> {
    match *e {
        Expr::Int(ref n) => Ok(Value::Int(*n)),
        Expr::Var(ref name) => env.lookup(name),
        Expr::BinOp(BinOp::Add, box ref lhs, box ref rhs) =>
            eval_binop!(lhs, rhs, env, |l, r| l+r, "add"),
        Expr::BinOp(BinOp::Sub, box ref lhs, box ref rhs) =>
            eval_binop!(lhs, rhs, env, |l, r| l-r, "sub"),
        Expr::BinOp(BinOp::Mult, box ref lhs, box ref rhs) =>
            eval_binop!(lhs, rhs, env, |l, r| l*r, "mult"),
        Expr::BinOp(BinOp::Div, box ref lhs, box ref rhs) =>
            eval_binop!(lhs, rhs, env, |l, r| l/r, "divide"),
        Expr::Func(ref x, box ref e) => Ok(Value::Func(x.clone(), box e.clone())),
        Expr::App(box ref e1, box ref e2) => {
            match (eval(e1, env)?, eval(e2, env)?) {
                (Value::Func(x, box body), v) => {
                    let new_env = env.push(x, v);
                    eval(&body, &new_env)
                },
                _ => Err(format!("can not app {} and {}", e1, e2)),
            }
        },
        Expr::Let(ref x, box ref init, box ref body) => {
            let arg = eval(init, env)?;
            let new_env = env.push(x.clone(), arg);
            eval(body, &new_env)
        },
    }
}


