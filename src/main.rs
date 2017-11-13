#![feature(box_patterns)]
#![feature(box_syntax)]

#[macro_use]
extern crate nom;

mod parser;
mod expr;
mod value;
mod typ;
mod eval;

use eval::Env;

use nom::IResult;

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    let mut input = String::new();
    match env::args().nth(1) {
        Some(filename) => {
            File::open(&filename).and_then(|mut f| {
                f.read_to_string(&mut input)
            }).unwrap();

            match parser::expr(input.as_bytes()) {
                IResult::Done(_, res) => {
                    println!("ok {}", res);
                    let v = eval::eval(&res, &Env::new());
                    println!("result: {:?}", v);
                }
                err => println!("error: {:?}", err),
            }
        },
        None => {
            loop {
                use std::io;
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                let line = line.trim();
                if line == "quit" || line == "end" {
                    break;
                }

                match parser::expr(input.as_bytes()) {
                    IResult::Done(_, res) => println!("ok {}", res),
                    err => println!("error: {:?}", err),
                }
            }
        },
    }
}
