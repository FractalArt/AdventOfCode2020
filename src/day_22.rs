//! This module contains the code
//! for the solution of the twenty-second day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/22).
use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<usize>;

/// Normal version of `Crab Combat` with the given decks.
pub fn task_1(data: &str) -> usize {
    let (deck_1, deck_2) = get_decks(data);

    crab_combat(deck_1, deck_2, false)
        .deck()
        .iter()
        .rev()
        .enumerate()
        .map(|(factor, card)| (factor + 1) * card)
        .sum()
}

/// Recursive version of `Crab Combat` with the given decks.
pub fn task_2(data: &str) -> usize {
    let (deck_1, deck_2) = get_decks(data);
    crab_combat(deck_1, deck_2, true)
        .deck()
        .iter()
        .rev()
        .enumerate()
        .map(|(factor, card)| (factor + 1) * card)
        .sum()
}

/// Get the player's deck from the input string.
fn get_decks(data: &str) -> (Deck, Deck) {
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

/// Track who the winner and its corresponding deck is.
#[derive(Debug, PartialEq)]
enum Winner {
    P1(Deck),
    P2(Deck),
}

impl Winner {
    /// Access the winner' s deck.
    fn deck(&self) -> &Deck {
        match self {
            Self::P1(val) => &val,
            Self::P2(val) => &val,
        }
    }
}

/// Implementation of `Crab Combat` The flag `recursive` indicates whether or not to use the
/// recursive version of the game.
fn crab_combat(mut deck_1: Deck, mut deck_2: Deck, recursive: bool) -> Winner {
    let (mut memory_1, mut memory_2) = (HashSet::new(), HashSet::new());

    loop {
        if memory_1.contains(&deck_1) || memory_2.contains(&deck_2) {
            return Winner::P1(deck_1);
        }

        memory_1.insert(deck_1.clone());
        memory_2.insert(deck_2.clone());

        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if deck_1.len() < card_1 || deck_2.len() < card_2 || !recursive {
            if card_1 > card_2 {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        } else {
            let new_deck_1: Deck = deck_1.iter().take(card_1).cloned().collect();
            let new_deck_2: Deck = deck_2.iter().take(card_2).cloned().collect();
            match crab_combat(new_deck_1.clone(), new_deck_2.clone(), recursive) {
                Winner::P1(_) => {
                    deck_1.push_back(card_1);
                    deck_1.push_back(card_2);
                }
                Winner::P2(_) => {
                    deck_2.push_back(card_2);
                    deck_2.push_back(card_1);
                }
            }
        }

        if deck_1.is_empty() {
            break Winner::P2(deck_2);
        }
        if deck_2.is_empty() {
            break Winner::P1(deck_1);
        }
    }
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

    #[test]
    fn test_day_22_task_2() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        assert_eq!(task_2(input), 291);
    }
}
