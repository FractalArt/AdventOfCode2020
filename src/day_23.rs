//! This module contains the code
//! for the solution of the twenty-third day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/23).
use std::collections::VecDeque;

/// Crab cups: Get the labels on the cups after cup 1 after `moves` moves.
pub fn task_1(input: &[usize], moves: usize) -> usize {
    let mut cups: VecDeque<usize> = input.iter().copied().collect();

    play_crab_cups(&mut cups, moves, 1, 9);

    cups.into_iter()
        .cycle()
        .skip_while(|&x| x != 1)
        .skip(1)
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, x)| 10usize.pow((input.len() - i - 2) as u32) * x)
        .sum::<usize>()
}

/// Play a huge `crab cups` game, find the cups next to cup 1 and return their product.
pub fn task_2(input: &[usize], moves: usize) -> usize {
    let mut cups: VecDeque<usize> = input.iter().copied().chain(10..=1000000).collect();
    play_crab_cups(&mut cups, moves, 1, 1_000_000);
    let pos = cups.iter().position(|&x| x == 1).unwrap() + 1;
    cups[pos-1]*cups[pos+1]
}

/// Implementation of the `crab cup` game for a given set of `cups`, a given number of `moves`
/// as well as the `min`imal and `max`imal cup numbers appearing in `cups`.
fn play_crab_cups(cups: &mut VecDeque<usize>, moves: usize, min: usize, max: usize) {
    let mut current_pos = 0;
    for mv in 1..=moves {
        if mv % 1000 == 0 {
            println!("Round: {}", mv);
        }
        let current = cups[current_pos];
        let mut three_next = Vec::with_capacity(3);
        let mut insert = current_pos + 1;
        for _ in 0..3 {
            if  insert == cups.len() {insert = 0};
            three_next.push(cups.remove(insert).unwrap());
        }
        
        let mut destination = if current == min { max } else { current - 1 };
        while three_next.contains(&destination) {
            destination = if destination == min { max } else { destination - 1 };
        }
        let pos = cups.iter().position(|x| x == &destination).unwrap();
        three_next.into_iter().rev().for_each(|x| cups.insert(pos + 1, x) );
        
        current_pos = cups.iter().position(|x| x == &current).unwrap() + 1;
        if current_pos == cups.len() {
            current_pos = 0;
        }
    }
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
