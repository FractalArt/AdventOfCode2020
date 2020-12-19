//! This module contains the code
//! for the solution of the eighteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/18).

/// Compute the sum of all the evaluated expressions in the input.
pub fn task_1(data: &[String]) -> usize {
    data.iter().map(|l| eval(l)).sum()
}

/// Compute the sum of all the evaluated expressions in the input.
/// This time addition has precedence over multiplication.
pub fn task_2(data: &[String]) -> usize {
    data.iter().map(|l| eval_add_prec(l)).sum()
}

/// The operators that are allowed in the expression.
enum Op {
    Add,
    Mul,
}

/// Evaluate an expression from its string representation.
fn eval<'a>(data: &str) -> usize {
    let mut cumulative = 0; // Accumulate the result of the operations
    let mut op = None; // Keep track of the last operator (+,*)
    let mut open_counter = 0; // Keep track of the currently opened brackets
    let mut tmp = vec![]; // Store expressions in brackets
    for c in data.chars() {
        match c {
            ' ' => continue,
            '(' => {
                if open_counter > 0 {
                    tmp.push(c)
                }
                open_counter += 1;
            }
            ')' => {
                open_counter -= 1;
                if open_counter == 0 {
                    match op {
                        None => cumulative = eval(&tmp.into_iter().collect::<String>()),
                        Some(Op::Add) => cumulative += eval(&tmp.into_iter().collect::<String>()),
                        Some(Op::Mul) => cumulative *= eval(&tmp.into_iter().collect::<String>()),
                    }
                    tmp = vec![];
                } else {
                    tmp.push(c)
                }
            }
            '*' => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    op = Some(Op::Mul)
                }
            }
            '+' => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    op = Some(Op::Add)
                }
            }
            c => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    match op {
                        None => cumulative = c.to_string().parse::<usize>().unwrap(),
                        Some(Op::Add) => cumulative += c.to_string().parse::<usize>().unwrap(),
                        Some(Op::Mul) => cumulative *= c.to_string().parse::<usize>().unwrap(),
                    }
                }
            }
        }
    }

    cumulative
}

/// Evaluate an expression from its string representation with addition having precedence over
/// multiplication.
fn eval_add_prec<'a>(data: &str) -> usize {
    let mut cumulative = 1; // Accumulate the result of the operations
    let mut op = None; // Keep track of the last operator (+,*)
    let mut open_counter = 0; // Keep track of the currently opened brackets
    let mut tmp = vec![]; // Store expressions in brackets
    let mut sum = 0;
    for c in data.chars() {
        match c {
            ' ' => continue,
            '(' => {
                if open_counter > 0 {
                    tmp.push(c)
                }
                open_counter += 1;
            }
            ')' => {
                open_counter -= 1;
                if open_counter == 0 {
                    match op {
                        None => sum = eval_add_prec(&tmp.into_iter().collect::<String>()),
                        Some(Op::Add) => sum += eval_add_prec(&tmp.into_iter().collect::<String>()),
                        Some(Op::Mul) => {
                            cumulative *= sum;
                            sum = eval_add_prec(&tmp.into_iter().collect::<String>())
                        }
                    }
                    tmp = vec![];
                } else {
                    tmp.push(c)
                }
            }
            '*' => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    op = Some(Op::Mul)
                }
            }
            '+' => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    op = Some(Op::Add)
                }
            }
            c => {
                if open_counter > 0 {
                    tmp.push(c)
                } else {
                    match op {
                        None => sum = c.to_string().parse::<usize>().unwrap(),
                        Some(Op::Add) => sum += c.to_string().parse::<usize>().unwrap(),
                        Some(Op::Mul) => {
                            cumulative *= sum;
                            sum = c.to_string().parse::<usize>().unwrap()
                        }
                    }
                }
            }
        }
    }

    cumulative * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval("3"), 3);
        assert_eq!(eval("3 + 2"), 5);
        assert_eq!(eval("(3 + 2)"), 5);
        assert_eq!(eval("3 + 5 * 2"), 16);
        assert_eq!(eval("3 + (5 * 2)"), 13);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_eval_prec_plus() {
        assert_eq!(eval_add_prec("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval_add_prec("2 * 3 + (4 * 5)"), 46);
    }
}
