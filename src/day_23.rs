//! This module contains the code
//! for the solution of the twenty-third day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/23).

/// Crab cups: Get the labels on the cups after cup 1 after `moves` moves.
pub fn task_1(input: &[usize], moves: usize) -> usize {
    let mut cup_circle = CupCircle::from_input(input, input.len());

    for _ in 0..moves {
        cup_circle.move_once();
    }

    cup_circle
        .get_config()
        .into_iter()
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
    let mut cup_circle = CupCircle::from_input(input, 1_000_000);
    for _ in 0..moves {
        cup_circle.move_once();
    }
    let final_config = cup_circle.get_config();
    let pos = final_config.iter().position(|&x| x == 1).unwrap();
    final_config[pos + 1] * final_config[pos + 2]
}

/// A circle of cups which is the focus of the `crab cups` game.
/// For each cup `c` (located at `c-1` in `neighbors`, i.e. the
/// cup labels are 1-based), we store the label of its neighboring
/// cup. The `current` cup under consideration is also tracked.
struct CupCircle {
    neighbors: Vec<usize>,
    current: usize,
}

impl CupCircle {
    /// Construct a `CupCircle` with `cups` cups from `input`.
    fn from_input(input: &[usize], cups: usize) -> CupCircle {
        let mut neighbors: Vec<usize> = (2..=cups + 1).into_iter().collect();
        neighbors[cups - 1] = input[0];
        for i in 0..input.len() - 1 {
            neighbors[input[i] - 1] = input[i + 1];
        }
        if cups > input.len() {
            neighbors[cups - 1] = input[0];
            neighbors[input[input.len() - 1] - 1] = input.len() + 1;
        } else {
            neighbors[input[input.len() - 1] - 1] = input[0];
        }

        Self {
            neighbors,
            current: input[0],
        }
    }

    /// Get the current configuration.
    fn get_config(&self) -> Vec<usize> {
        let mut cup = self.current;
        let mut v = Vec::with_capacity(self.neighbors.len());
        for _ in 0..self.neighbors.len() {
            v.push(cup);

            cup = if cup == 0 {
                self.neighbors.len()
            } else {
                self.neighbors[cup - 1]
            };
        }

        v
    }

    /// Perform one movement.
    fn move_once(&mut self) {
        // Get next three neighbours
        let n1 = self.neighbors[self.current - 1];
        let n2 = self.neighbors[n1 - 1];
        let n3 = self.neighbors[n2 - 1];

        let mut destination = self.current - 1;
        if destination == 0 {
            destination = self.neighbors.len();
        }
        while destination == n1 || destination == n2 || destination == n3 {
            destination -= 1;
            if destination == 0 {
                destination = self.neighbors.len()
            }
        }

        self.neighbors[self.current - 1] = self.neighbors[n3 - 1];
        let tmp = self.neighbors[destination - 1];
        self.neighbors[destination - 1] = n1;
        self.neighbors[n3 - 1] = tmp;
        self.current = self.neighbors[self.current - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cup_circle() {
        let mut cup_circle = CupCircle::from_input(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 9);
        assert_eq!(cup_circle.neighbors, vec![2, 5, 8, 6, 4, 7, 3, 9, 1]);
        assert_eq!(cup_circle.current, 3);

        cup_circle.move_once();

        assert_eq!(cup_circle.neighbors, vec![5, 8, 2, 6, 4, 7, 3, 9, 1]);
        assert_eq!(cup_circle.current, 2);
        assert_eq!(cup_circle.get_config(), vec![2, 8, 9, 1, 5, 4, 6, 7, 3]);

        let cup_circle = CupCircle::from_input(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 20);
        assert_eq!(
            cup_circle.neighbors,
            vec![2, 5, 8, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3]
        );
    }

    #[test]
    fn test_day_23_task_1() {
        assert_eq!(task_1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 10), 92658374);
        assert_eq!(task_1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 100), 67384529);
    }

    #[test]
    fn test_day_23_task_2() {
        assert_eq!(
            task_2(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 10_000_000),
            149245887792
        );
    }
}
