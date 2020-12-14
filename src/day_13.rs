//! This module contains the code
//! for the solution of the thirteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/13).

/// Compute the product of bus number and waiting time for the
/// bus that minimizes waiting time.
pub fn task_1(data: &str) -> usize {
    let split = data.split_whitespace().collect::<Vec<_>>();
    assert_eq!(split.len(), 2);

    let time: usize = split[0].parse().unwrap();
    let result = split[1]
        .split(",")
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|x| (x, (time / x) * x + if time % x != 0 { x } else { 0 } - time))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    result.0 * result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_task_1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(task_1(&input), 295);
    }
}
