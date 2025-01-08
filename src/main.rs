#![warn(dead_code)]

mod arg_parsing;
mod expression;
mod maths;

use arg_parsing::Args;
use clap::Parser;
use expression::try_solve;

type Int = i32;

fn main() {
    let args = Args::parse();
    let numbers = args.into();

    if let Some(expr) = try_solve(&numbers) {
        println!("Solution found: {}", expr);
    } else {
        println!("No solution found!");
    }
}
