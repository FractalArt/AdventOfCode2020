//! This module contains the code
//! for the solution of the first day challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/1).

use itertools::Itertools;

/// The solution to task 1 of day 1
///
/// Task: Find the combination of `elements_to_sum` values whose sum
/// is equal to `sum_target` and return their product.
///
/// The input values are stored in the vector `data`.
pub fn day_1(data: &[u32], elements_to_sum: usize, sum_target: u32) -> u32 {
    data.iter()
        .combinations(elements_to_sum)
        .filter(|v| v.iter().copied().sum::<u32>() == sum_target)
        .map(|v| v.into_iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        assert_eq!(514579, day_1(&[1721, 979, 366, 299, 675, 1456], 2, 2020));
    }
}
