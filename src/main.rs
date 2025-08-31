use clap::Parser;
use p24::{arg_parsing::Args, expression::try_solve};

fn main() {
    let args = Args::parse();
    let numbers = args.into();

    if let Some(expr) = try_solve(&numbers) {
        println!("Solution found: {}", expr);
    } else {
        println!("No solution found!");
    }
}
