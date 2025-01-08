use crate::maths::rational::Rational;
use crate::Int;

pub fn evaluate_expression(expr: &str, numbers: &[Int; 4]) -> Option<Int> {
    let mut stack: Vec<Rational> = Vec::with_capacity(4);

    for next_char in expr.bytes() {
        if (b'1'..=b'4').contains(&next_char) {
            let index = (next_char - b'1') as usize;
            stack.push(Rational::from_int(numbers[index]));
        } else {
            let op2 = stack.pop()?;
            let op1 = stack.pop()?;
            let result = match next_char {
                b'+' => op1.add(&op2),
                b'-' => op1.sub(&op2),
                b'*' => op1.mul(&op2),
                b'/' => op1.div(&op2)?,
                _ => {
                    return None;
                }
            };
            stack.push(result);
        }
    }

    if stack.len() > 1 {
        None
    } else {
        let result = stack.pop()?;
        result.evaluate_int()
    }
}

#[test]
fn evaluate_expression_test() {
    let numbers: [Int; 4] = [5, 5, 5, 1];
    let expr = "142/-3*";
    assert_eq!(Some(24), evaluate_expression(expr, &numbers));
}
