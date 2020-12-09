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

    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;
    let mut index: usize = 0;

    loop {
        match instructions[index] {
            Instruction::Nop => index += 1,
            Instruction::Jmp(val) => index = index + val as usize,

            Instruction::Acc(val) => {
                accumulator += val;
                index += 1;
            }
        }
        if !visited.insert(index) {
            break accumulator;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop,
    Acc(isize),
    Jmp(isize),
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
        ("nop", _, _) => Instruction::Nop,
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
        assert_eq!(parse_instruction("nop +0"), Nop);
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
}
