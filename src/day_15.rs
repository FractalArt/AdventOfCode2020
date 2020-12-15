//! This module contains the code
//! for the solution of the fifteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/15).
use std::collections::HashMap;

/// Compute the final number said in the game.
pub fn task_1(data: &[usize], end: usize) -> usize {
    assert!(end > data.len());
    let mut memory = HashMap::new();
    let mut previous = data[data.len() - 1];

    // Insert all the starting numbers into the memory, except for the last one
    // which is stored in `previous`.
    for i in 0..data.len() - 1 {
        memory.insert(data[i], i + 1);
    }

    for iter in data.len() + 1..=end {
        match memory.insert(previous, iter - 1) {
            None => previous = 0,
            Some(val) => previous = iter - 1 - val,
        }
    }

    previous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15_task_1() {
        let input = [0, 3, 6];
        assert_eq!(task_1(&input, 4), 0);
        assert_eq!(task_1(&input, 5), 3);
        assert_eq!(task_1(&input, 6), 3);
        assert_eq!(task_1(&input, 7), 1);
        assert_eq!(task_1(&input, 8), 0);
        assert_eq!(task_1(&input, 9), 4);
        assert_eq!(task_1(&input, 10), 0);
        assert_eq!(task_1(&input, 2020), 436);
    }
}
