//! This module contains the code
//! for the solution of the twenty-third day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/23).

/// Crab cups: Get the labels on the cups after cup 1 after `moves` moves.
pub fn task_1(input: &[usize], moves: usize) -> usize {
    let mut cups: Vec<usize> = input.into_iter().copied().collect();
    let max = cups.iter().max().unwrap().clone();
    let min = cups.iter().min().unwrap().clone();

    for _ in 1..=moves {
        let three_next = vec![cups[1], cups[2], cups[3]];
        let mut target = if cups[0] == min { max } else { cups[0] - 1 };
        while three_next.contains(&target) {
            target = if target == min { max } else { target - 1 };
        }
        let pos = cups.iter().position(|&x| x == target).unwrap();
        let intermediate = cups[4..=pos].iter().map(|x| x.clone()).collect::<Vec<_>>();
        let end = cups[pos + 1..].iter().copied().collect();
        cups = vec![intermediate, three_next, end, vec![cups[0]]]
            .into_iter()
            .flat_map(|v| v.into_iter())
            .collect();
    }

    cups.into_iter()
        .cycle()
        .skip_while(|&x| x != 1)
        .skip(1)
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, x)| 10usize.pow((input.len() - i - 2) as u32) * x)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_23_task_1() {
        assert_eq!(task_1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 10), 92658374);
        assert_eq!(task_1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 100), 67384529);
    }
}
