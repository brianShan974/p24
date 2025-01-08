use crate::maths::rational::Rational;
use crate::Int;

const DIGITS: [&str; 4] = ["1", "2", "3", "4"];
const OPS: [&str; 4] = ["+", "-", "*", "/"];

const SHAPES: &[&str] = &["NNNNOOO", "NNNOONO", "NNNONOO", "NNONONO", "NNONNOO"];

pub fn try_solve(numbers: &[Int; 4]) -> Option<String> {
    for shape in SHAPES {
        let exprs = gen_exprs(shape)?;
        for expr in exprs {
            if evaluate_expression(&expr, numbers) == Some(24) {
                return postfix_to_infix(&expr, numbers);
            }
        }
    }
    None
}

fn evaluate_expression(expr: &str, numbers: &[Int; 4]) -> Option<Int> {
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

fn gen_exprs(shape: &str) -> Option<Vec<String>> {
    let mut excluded: Vec<char> = Vec::with_capacity(4);
    gen_exprs_recursive(shape, &mut excluded, 0)
}

fn gen_exprs_recursive(
    shape: &str,
    excluded: &mut Vec<char>,
    starting_index: usize,
) -> Option<Vec<String>> {
    if starting_index == shape.len() - 1 {
        let first_char = shape.chars().nth(starting_index)?;
        if first_char == 'O' {
            return Some(vec![
                String::from("+"),
                String::from("-"),
                String::from("*"),
                String::from("/"),
            ]);
        } else {
            return None;
        }
    }

    let next_char = shape.chars().nth(starting_index)?;
    let mut result = Vec::new();
    match next_char {
        'N' => {
            for digit in DIGITS {
                let digit_char = digit.chars().next()?;
                if !excluded.contains(&digit_char) {
                    excluded.push(digit_char);
                    let r = gen_exprs_recursive(shape, excluded, starting_index + 1)?;
                    for s in r.iter() {
                        let mut s = s.clone();
                        s.insert_str(0, digit);
                        result.push(s);
                    }
                    excluded.pop();
                }
            }
            Some(result)
        }
        'O' => {
            let r = gen_exprs_recursive(shape, excluded, starting_index + 1)?;
            for op in OPS {
                for s in r.iter() {
                    let mut s = s.clone();
                    s.insert_str(0, op);
                    result.push(s);
                }
            }
            Some(result)
        }
        _ => None,
    }
}

fn postfix_to_infix(expr: &str, numbers: &[Int; 4]) -> Option<String> {
    let mut stack: Vec<String> = Vec::with_capacity(4);

    for next_char in expr.bytes() {
        if (b'1'..=b'4').contains(&next_char) {
            let index = (next_char - b'1') as usize;
            stack.push(numbers[index].to_string());
        } else {
            let op2 = stack.pop()?;
            let op1 = stack.pop()?;
            let result = match next_char {
                b'+' => format!("({} + {})", op1, op2),
                b'-' => format!("({} - {})", op1, op2),
                b'*' => format!("{} * {}", op1, op2),
                b'/' => format!("{} / {}", op1, op2),
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
        stack.pop()
    }
}

#[test]
fn evaluate_expression_test() {
    let numbers: [Int; 4] = [5, 5, 5, 1];
    let expr = "142/-3*";
    assert_eq!(Some(24), evaluate_expression(expr, &numbers));
}

#[test]
fn gen_exprs_test() {
    let shape = "NNNNOOO";
    let exprs = gen_exprs(shape).unwrap();
    assert!(exprs.contains(&"1234-++".to_string()))
}

#[test]
fn postfix_to_infix_test() {
    let numbers: [Int; 4] = [5, 5, 5, 1];
    let expr = "142/-3*";

    let result = "(5 - 1 / 5) * 5".to_string();
    assert_eq!(result, postfix_to_infix(expr, &numbers).unwrap())
}
