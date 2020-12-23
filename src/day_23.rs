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
    cups[pos + 1] * cups[pos + 2]
}

/// Implementation of the `crab cup` game for a given set of `cups`, a given number of `moves`
/// as well as the `min`imal and `max`imal cup numbers appearing in `cups`.
fn play_crab_cups(cups: &mut VecDeque<usize>, moves: usize, min: usize, max: usize) {
    let mut current_pos = 0;
    for mv in 1..=moves {
        // println!("[{}]: {:?}", mv, cups);
        if mv % 1000 == 0 {
            println!("{}", mv);
        }
        // Collect the next three elements
        let current = cups[current_pos];

        // Determine the destination position
        let mut destination = if current == min { max } else { current - 1 };
        while cups
            .iter()
            .cycle()
            .skip(current_pos + 1)
            .cycle()
            .take(3)
            .find(|&x| x == &destination)
            .is_some()
        {
            destination = if destination == min {
                max
            } else {
                destination - 1
            };
        }
        let pos = cups.iter().position(|x| x == &destination).unwrap();

        // Find the other elements to be replaced
        let replace = if pos < current_pos {
            let len_between = current_pos - pos;
            cups.iter()
                .cycle()
                .skip(current_pos + 1)
                .take(3)
                .chain(cups.iter().cycle().skip(pos + 1).take(len_between))
                .copied()
                .collect::<Vec<_>>()
        } else {
            let len_between = pos - current_pos - 3;
            cups.iter()
                .cycle()
                .skip(current_pos + 4)
                .take(len_between)
                .chain(cups.iter().cycle().skip(current_pos + 1).take(3))
                .copied()
                .collect::<Vec<_>>()
        };

        // Perform the replacements
        let mut index = std::cmp::min(current_pos, pos);
        for i in 0..replace.len() {
            index += 1;
            if index == cups.len() {
                index = 0
            }
            let tmp = replace[i];
            cups[index] = tmp;
        }

        // TODO: Compute the current position explicitly to avoid finding it
        // which might be costly
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
