mod expression;
mod maths;

use expression::try_solve;

type Int = i32;
type UInt = u32;
type Float = f32;

fn main() {
    let numbers = [5, 5, 5, 1];
    if let Some(expr) = try_solve(&numbers) {
        println!("Solution found: {}", expr);
    } else {
        println!("No solution found!");
    }
}
