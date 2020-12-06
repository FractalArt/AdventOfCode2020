//! This module contains the code
//! for the solution of the sixth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/6).
use std::collections::HashSet;

/// Compute the sum of the questions answered with yes per group.
pub fn task_1(data: &str) -> usize {
    data.split("\n\n")
        .map(|s| s)
        .map(|s| count_questions_yes_answers(s))
        .sum()
}

/// Compute the sum of the number of questions that have been answered with yes by __all__ group members.
pub fn task_2(data: &str) -> usize {
    data.split("\n\n")
        .map(|s| s)
        .map(|s| count_questions_all_yes_answers(s))
        .sum()
}

/// Compute the number of questions answered with yes per group.
fn count_questions_yes_answers(s: &str) -> usize {
    s.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<HashSet<_>>()
        .len()
}

/// Compute the number of questions answered with yes by __all__ people in a group.
fn count_questions_all_yes_answers(input: &str) -> usize {
    let members = input.split_whitespace().map(|s|s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    members[0]
        .iter()
        .filter(|c| members.iter().all(|m| m.contains(c)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_questions_yes_answers() {
        assert_eq!(count_questions_yes_answers("abc"), 3);
        assert_eq!(count_questions_yes_answers("a\nb\nc"), 3);
        assert_eq!(count_questions_yes_answers("a\na\na"), 1);
    }

    #[test]
    fn test_count_questions_all_yes_answers() {
        assert_eq!(count_questions_all_yes_answers("qepdrhamt\nifnd\nnxfdy"), 1);
        assert_eq!(count_questions_all_yes_answers("a\nb\nc"), 0);
        assert_eq!(count_questions_all_yes_answers("a\na\na"), 1);
    }

    #[test]
    fn test_day_6_task_1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(task_1(&input), 11);
    }

    #[test]
    fn test_day_6_task_2() {
        let input_1 = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let input_2 = "
        qepdrhamt
        ifnd
        nxfdy";
        assert_eq!(task_2(&input_1), 6);
        assert_eq!(task_2(&input_2), 1);
    }
}
