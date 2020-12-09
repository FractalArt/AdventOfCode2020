//! This module contains the code
//! for the solution of the eight day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/8).
use regex::Regex;
use std::collections::HashSet;

/// Find the state of the accumulator before entering the infinite loop.
pub fn task_1(data: &Vec<String>) -> isize {
    let instructions: Vec<Instruction> = data.iter().map(|l| parse_instruction(l)).collect();

    match helper(&instructions) {
        ExitOn::Loop(val) => val,
        _ => panic!("Task 1 should finish"),
    }
}

/// Fix the bug and compute the state of the accumulator after the program finishes.
pub fn task_2(data: &Vec<String>) -> isize {
    let instructions: Vec<Instruction> = data.iter().map(|l| parse_instruction(l)).collect();
    // Get the positions of all jmp and nop instructions
    let possible_swaps = instructions
        .iter()
        .enumerate()
        .filter(|(_, op)| match op {
            Instruction::Jmp(_) | Instruction::Nop(_) => true,
            _ => false,
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // Try out all possible replacements
    possible_swaps
        .into_iter()
        .map(|index| {
            let tmp = instructions
                .iter()
                .enumerate()
                .map(|(i, op)| if i == index { op.swap() } else { op.clone() })
                .collect();
            helper(&tmp)
        })
        .find_map(|i| match i {
            ExitOn::Loop(_) => None,
            ExitOn::Finish(val) => Some(val),
        })
        .expect("Task 2 should have a solution!")
}

/// Check whether the given instructions lead to a loop or not.
/// Return the value of the accumulator at the end of the program
/// or when the loop starts, wrapped in an `ExitOn` enum.
fn helper(instructions: &Vec<Instruction>) -> ExitOn {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut index: isize = 0;

    loop {
        match instructions[index as usize] {
            Instruction::Nop(_) => index += 1,
            Instruction::Jmp(val) => index = index + val,
            Instruction::Acc(val) => {
                accumulator += val;
                index += 1;
            }
        }
        if !visited.insert(index as usize) {
            break ExitOn::Loop(accumulator);
        }
        if index as usize == instructions.len() {
            break ExitOn::Finish(accumulator);
        }
    }
}

/// Indicate whether a program finishes or enters a loop.
/// Store the value of the accumulator at the end of the
/// program or the beginning of the loop.
#[derive(Debug, PartialEq)]
enum ExitOn {
    Finish(isize),
    Loop(isize),
}

/// Enumerate the instructions
#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    /// Swap the `Nop` and `Jmp` operations, leave `Acc` untouched.
    fn swap(&self) -> Self {
        match self {
            Self::Nop(val) => Self::Jmp(*val),
            Self::Acc(val) => Self::Acc(*val),
            Self::Jmp(val) => Self::Nop(*val),
        }
    }
}

/// Parse an instruction from the input file.
fn parse_instruction<'a>(code: &'a str) -> Instruction {
    lazy_static::lazy_static! {
        static ref INSTRUCTION: Regex = Regex::new(r"^(\w\w\w) ([+|-])(\d+)$").unwrap();
    }
    let captures = INSTRUCTION.captures(code).unwrap();
    let instruction = captures.get(1).unwrap().as_str();
    let sign = captures.get(2).unwrap().as_str();
    let integer = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();

    match (instruction, sign, integer) {
        ("nop", "+", val) => Instruction::Nop(val),
        ("nop", "-", val) => Instruction::Nop(-val),
        ("acc", "+", val) => Instruction::Acc(val),
        ("acc", "-", val) => Instruction::Acc(-val),
        ("jmp", "+", val) => Instruction::Jmp(val),
        ("jmp", "-", val) => Instruction::Jmp(-val),
        (_, _, _) => panic!("Got wrong instruction."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        use Instruction::*;
        assert_eq!(parse_instruction("nop +0"), Nop(0));
        assert_eq!(parse_instruction("acc +1"), Acc(1));
        assert_eq!(parse_instruction("jmp +4"), Jmp(4));
        assert_eq!(parse_instruction("jmp -4"), Jmp(-4));
        assert_eq!(parse_instruction("jmp -346"), Jmp(-346));
    }

    #[test]
    fn test_day_8_task_1() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        assert_eq!(task_1(&input), 5);
    }

    #[test]
    fn test_day_8_task_2() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        assert_eq!(task_2(&input), 8);
    }
}
