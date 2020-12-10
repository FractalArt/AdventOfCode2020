//! This module contains the code
//! for the solution of the ninth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/9).
use itertools::Itertools;

/// Find the number which does not correspond to the sum of any pair
/// of numbers belonging to the previous `memory` numbers.
pub fn task_1(data: &[u64], memory: usize) -> u64 {
    data.windows(memory + 1)
        .find_map(|w| {
            if let false = w[0..memory]
                .iter()
                .combinations(2)
                .any(|t| t[0] + t[1] == w[memory])
            {
                Some(w[memory].clone())
            } else {
                None
            }
        })
        .unwrap()
}

/// Compute the sum of the minimum and maximum range of continuous numbers
/// whose sum equals the invalid number found in task 1.
pub fn task_2(data: &[u64], memory: usize) -> u64 {
    let invalid = task_1(data, memory);

    let w = (2..)
        .into_iter()
        .find_map(|i| {
            data.windows(i).find_map(|w| {
                if w.iter().sum::<u64>() == invalid {
                    Some(w.iter().sorted().collect::<Vec<_>>())
                } else {
                    None
                }
            })
        })
        .unwrap();

    w[0] + w[w.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9_task_1() {
        let input = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(task_1(&input, 5), 127);
    }

    #[test]
    fn test_day_9_task_2() {
        let input = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(task_2(&input, 5), 62);
    }
}
