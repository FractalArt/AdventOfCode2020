//! This module contains the code
//! for the solution of the tenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/10).
use itertools::Itertools;
use std::collections::HashMap;

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

/// Compute the number of valid adaptor chains.
pub fn task_2(data: &[u32]) -> usize {
    let sorted = data.iter().sorted().map(|x| *x).collect::<Vec<_>>();
    valid_next(0, 0, &sorted, &mut HashMap::new())
}

/// Compute the number of valid adaptor combinations that can be
/// obtained starting from the index `start` in `data`. The previous
/// value `prev` is in general the entry in `data` with index `start-1`,
/// except for the first call where it is `0`, the charging outlet.
/// This parameter could be omitted by including 0 in `data` manually.
fn valid_next(prev: u32, start: usize, data: &[u32], mut cache: &mut HashMap<u32, usize>) -> usize {
    if start == data.len() {
        return 1;
    }
    if cache.contains_key(&prev) {
        return cache[&prev];
    }

    let sum = (start..data.len())
        .into_iter()
        .filter(|&index| data[index] - prev <= 3)
        .map(|index| valid_next(data[index], index + 1, data, &mut cache))
        .sum();

    cache.insert(prev, sum);
    sum
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

    #[test]
    fn test_valid_next() {
        let mut input_1 = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input_1.sort();
        assert_eq!(valid_next(0, 0, &input_1, &mut HashMap::new()), 8);

        let mut input_2 = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input_2.sort();
        assert_eq!(valid_next(0, 0, &input_2, &mut HashMap::new()), 19208);
    }
}
