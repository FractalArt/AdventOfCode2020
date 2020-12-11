//! This module contains the code
//! for the solution of the eleventh day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/11).
use itertools::Itertools;
use std::cmp::{max, min};

/// Count the number of occupied seats after the equilibrium state has been
/// reached and the statuses of all seats remain constant, i.e. no people move
/// around.
pub fn task_1(data: &str) -> usize {
    let mut room: Room = data.into();
    while room.update() {}
    room.occupied()
}

/// The different states a spot can have.
#[derive(Debug, Clone)]
enum Spot {
    Floor,
    OccupiedSeat,
    EmptySeat,
}

/// A `Room` is characterized by the state of all its spots and their
/// dimensions.
#[derive(Debug)]
pub struct Room {
    spots: Vec<Vec<Spot>>,
    rows: usize,
    cols: usize,
}

// Construct a `Room` from a `&str`.
impl From<&str> for Room {
    fn from(raw: &str) -> Self {
        let v = raw
            .split_whitespace()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Spot::Floor,
                        'L' => Spot::EmptySeat,
                        _ => Spot::OccupiedSeat,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            rows: v.len(),
            cols: v[0].len(),
            spots: v,
        }
    }
}

impl Room {
    /// Count how many seats, adjacent to the one located at `coords` are occupied.
    pub fn adjacent_occupied(&self, coord: (usize, usize)) -> u32 {
        let mut sum = 0;

        for row in max(0, coord.0 as isize - 1) as usize..min(self.rows, coord.0 + 2) {
            for col in max(0, coord.1 as isize - 1) as usize..min(self.cols, coord.1 + 2) {
                if row == coord.0 && col == coord.1 {
                    continue;
                }
                if let Spot::OccupiedSeat = self.spots[row][col] {
                    sum += 1
                }
            }
        }

        sum
    }

    /// Update seat statuses and return the number of seats that have been changed
    pub fn update(&mut self) -> bool {
        let mut changes = false;

        self.spots = self
            .spots
            .iter()
            .enumerate()
            .map(|(ri, r)| {
                r.iter()
                    .enumerate()
                    .map(|(ci, c)| {
                        let occ = self.adjacent_occupied((ri, ci));
                        match c {
                            Spot::OccupiedSeat if occ >= 4 => {
                                changes = true;
                                Spot::EmptySeat
                            }
                            Spot::EmptySeat if occ == 0 => {
                                changes = true;
                                Spot::OccupiedSeat
                            }
                            _ => c.clone(),
                        }
                    })
                    .collect()
            })
            .collect();

        changes
    }

    /// Count how many seats ins the room are occupied
    fn occupied(&self) -> usize {
        self.spots
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|c| {
                        if let Spot::OccupiedSeat = c {
                            true
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum()
    }

    /// This function is only needed for unit testing, to compare the
    /// internal state of the room to the examples provided in the challenge.
    fn _to_string(&self) -> String {
        self.spots
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| match s {
                        Spot::OccupiedSeat => "#",
                        Spot::EmptySeat => "L",
                        Spot::Floor => ".",
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room() {
        let room: Room = "#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##".into();

        assert_eq!(room.rows, 10);
        assert_eq!(room.cols, 10);
        assert_eq!(room.adjacent_occupied((0, 0)), 1);
        assert_eq!(room.adjacent_occupied((1, 1)), 2);
    }

    #[test]
    fn test_room_update() {
        let start = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let mut room: Room = start.into();

        let iteration_1 = "#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##";
        assert_eq!(&room._to_string(), start);
        room.update();

        assert_eq!(&room._to_string(), iteration_1);
    }
}
