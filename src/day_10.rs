//! This module contains the code
//! for the solution of the tenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/10).
use itertools::Itertools;

/// Compute the product of number of 1-jolt and 3-jolt differences
/// in a chain connecting all adapters.
pub fn task_1(data: &[u32]) -> u32 {
    let acc = data
        .iter()
        .sorted()
        .fold((0, 0, 0), |acc, x| match x - acc.2 {
            1 => (acc.0 + 1, acc.1, *x),
            _ => (acc.0, acc.1 + 1, *x), // Assume that only difference 1 and 2 can appear
        });
    // Last one is always 3 jolts higher
    acc.0 * (acc.1 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_task_1() {
        let input = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(task_1(&input), 220);
    }
}
