use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

/// Read the data from the input file
pub fn read_data<P: AsRef<Path>>(path: P) -> io::Result<Vec<u32>> {
    let f = File::open(path)?;
    let vec = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().trim().parse::<u32>().unwrap())
        .collect();

    Ok(vec)
}

/// The solution to task 1 of day 1
///
/// Task: Find the combination of `elements_to_sum` values whose sum
/// is equal to `sum_target` and return their product.
///
/// The input values are stored in the vector `data`.
pub fn day_1(data: &Vec<u32>, elements_to_sum: usize, sum_target: u32) -> u32 {
    data.iter()
        .to_owned()
        .combinations(elements_to_sum)
        .filter(|v| v.into_iter().fold(0u32, |a, &&b| a + b) == sum_target)
        .map(|v| v.into_iter().product::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        assert_eq!(
            514579,
            day_1(&vec![1721, 979, 366, 299, 675, 1456], 2, 2020)
        );
    }
}