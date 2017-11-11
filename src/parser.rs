use std::str::{self, FromStr};
use std::ops::{Range, RangeFrom, RangeTo};
use nom::{IResult, digit, alpha, multispace, Slice, InputIter, AsChar, InputLength, Needed, ErrorKind};

use expr::BinOp;
use expr::Expr;


fn valid_ident_chars<T>(input: T) -> IResult<T, T> where
    T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
    T: InputIter + InputLength,
    <T as InputIter>::Item: AsChar
{
    let alp: Vec<_> = (b'a' .. b'Z').collect();
    let num: Vec<_> = (b'0' .. b'9').collect();
    let valid_chars = format!("{}{}_!?'",
        String::from_utf8(alp).unwrap(),
        String::from_utf8(num).unwrap());

    let input_length = input.input_len();
    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }

    for (idx, item) in input.iter_indices() {
        let item = item.as_char();
        if !valid_chars.chars().find(|c| *c == item).is_some() {
            if idx == 0 {
                return IResult::Error(ErrorKind::Custom(42))
            } else {
                return IResult::Done(input.slice(idx..), input.slice(0..idx))
            }
        }
    }
    IResult::Done(input.slice(input_length..), input)
}

named!(identifier<String>, do_parse!(
    head: map_res!(alpha, str::from_utf8) >>
    tail: opt!(map_res!(valid_ident_chars, str::from_utf8)) >>
    opt!(multispace) >>
    (format!("{}{}", head, if let Some(x) = tail { x } else { "" }))
));

named!(literal_int<Expr>, map!(
    map_res!(
        map_res!(
            delimited!(opt!(multispace), digit, opt!(multispace)),
            str::from_utf8
            ),
        FromStr::from_str
        ),
    Expr::Int
));

named!(literal_func<Expr>, do_parse!(
    tag!("func") >>
    multispace >>
    x: identifier >>
    tag!("->") >>
    opt!(multispace) >>
    e: expr >>
    (Expr::Func("x".to_string(), box e))
));

named!(variable_expr<Expr>, map!(
    identifier,
    |x| Expr::Var(x)
));

named!(primary_expr<Expr>, alt!(
    literal_int |
    literal_func |
    variable_expr
));

named!(apply_expr<Expr>, do_parse!(
    init: primary_expr >>
    remainder: many0!(
        do_parse!(
            tag!("@") >>
            opt!(multispace) >>
            e: primary_expr >>
            (e))) >>
    (remainder.into_iter().fold(init, |acc, rhs| Expr::App(box acc, box rhs)))
));

named!(multive_expr<Expr>, do_parse!(
    init: apply_expr >>
    remainder: many0!(
        alt!(
            do_parse!(
                tag!("*") >>
                opt!(multispace) >>
                rhs: apply_expr >>
                (BinOp::Mult, rhs)) |
            do_parse!(
                tag!("/") >>
                opt!(multispace) >>
                rhs: apply_expr >>
                (BinOp::Div, rhs))
            )
        ) >>
    (fold_exprs(init, remainder))
));

named!(additive_expr<Expr>, do_parse!(
    init: multive_expr >>
    remainder: many0!(
        alt!(
            do_parse!(
                tag!("+") >>
                opt!(multispace) >>
                rhs: multive_expr >>
                (BinOp::Add, rhs)) |
            do_parse!(
                tag!("-") >>
                opt!(multispace) >>
                rhs: multive_expr >>
                (BinOp::Sub, rhs))
            )
        ) >>
    (fold_exprs(init, remainder))
));

named!(let_expr<Expr>, alt!(
    do_parse!(
        tag!("let") >>
        multispace >>
        x: identifier >>
        tag!("=") >>
        opt!(multispace) >>
        e1: expr >>
        tag!("in") >>
        multispace >>
        e2: expr >>
        (Expr::Let(x, box e1, box e2))) |
    additive_expr
));

named!(pub expr<Expr>, do_parse!(
    opt!(multispace) >>
    e: let_expr >>
    (e)
));

fn fold_exprs(init: Expr, remainder: Vec<(BinOp, Expr)>) -> Expr {
    remainder.into_iter().fold(init, |acc, (op, rhs)| {
        Expr::BinOp(op, box acc, box rhs)
    })
}

