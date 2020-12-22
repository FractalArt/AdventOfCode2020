//! This module contains the code
//! for the solution of the twenty-second day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/22).
use std::collections::VecDeque;

/// Compute the sum of the scores of the cards in the winning player's
/// deck. The score of a card is its value multiplied by its position
/// (where last position has score 1, second but last has score 2...).
pub fn task_1(data: &str) -> usize {
    let (mut deck_1, mut deck_2) = get_decks(data);
    let winner = loop {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }

        if deck_1.is_empty() {
            break deck_2;
        }
        if deck_2.is_empty() {
            break deck_1;
        }
    };

    winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(factor, card)| (factor + 1) * card)
        .sum()
}

/// Get the player's deck from the input string.
fn get_decks(data: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut split = data.split("\n\n");
    (
        split
            .next()
            .unwrap()
            .split('\n')
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        split
            .next()
            .unwrap()
            .split('\n')
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_decks() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        let (mut deck_1, mut deck_2) = get_decks(&input);

        assert_eq!(deck_1.len(), 5);
        assert_eq!(deck_2.len(), 5);

        assert_eq!(deck_1.pop_front(), Some(9));
        assert_eq!(deck_1.pop_front(), Some(2));
        assert_eq!(deck_1.pop_front(), Some(6));
        assert_eq!(deck_1.pop_front(), Some(3));
        assert_eq!(deck_1.pop_front(), Some(1));
        assert_eq!(deck_1.pop_front(), None);

        assert_eq!(deck_2.pop_front(), Some(5));
        assert_eq!(deck_2.pop_front(), Some(8));
        assert_eq!(deck_2.pop_front(), Some(4));
        assert_eq!(deck_2.pop_front(), Some(7));
        assert_eq!(deck_2.pop_front(), Some(10));
        assert_eq!(deck_2.pop_front(), None);
    }

    #[test]
    fn test_day_22_task_1() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(task_1(input), 306);
    }
}
