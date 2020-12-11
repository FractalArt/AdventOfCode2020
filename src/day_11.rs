//! This module contains the code
//! for the solution of the eleventh day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/11).
use itertools::Itertools;
use std::cmp::{max, min};

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
];

/// Count the number of occupied seats after the equilibrium state has been
/// reached and the statuses of all seats remain constant, i.e. no people move
/// around. One can specify the strategy to use for defining which seats shall
/// be considered in the updating process and what the threshold for surrounding
/// occupied seats is.
pub fn task_1_2(data: &str, strategy: &Strategy, threshold: u32) -> usize {
    let mut room: Room = data.into();
    while room.update(strategy, threshold) {}
    room.occupied()
}

/// Choose whether to consider adjacent occupied seats or visible occupied seats.
#[derive(Debug)]
pub enum Strategy {
    Adjacent,
    Visible,
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
struct Room {
    spots: Vec<Vec<Spot>>,
    rows: usize,
    cols: usize,
}

// Construct a `Room` from a `&str`.
impl From<&str> for Room {
    fn from(string: &str) -> Self {
        let v = string
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
    fn adjacent_occupied(&self, coord: (usize, usize)) -> u32 {
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

    /// Find an occupied seat in the direction `dir` starting from seat located at `coord`.
    /// We use that the directions are either diagonal, horizontal or vertical to only
    /// use one loop variable `i` in order to describe two components. Using two variables would
    /// lead to an infinite loop when e.g. dir.1 = 0. Example:
    ///      (row, col) = (`coord.0` + `i`*`dir.0`, `coord.1` + `i`*`dir.1`).
    /// If `dir.0` and `dir.1` are non-zero, we are on the diagonal, for `dir.0`=0, we move
    /// horizontally and for `dir.1` zero, we move vertically.
    fn find_occupied_in_direction(&self, coord: (usize, usize), dir: (isize, isize)) -> bool {
        let row = coord.0 as isize;
        let col = coord.1 as isize;
        for i in 1.. {
            if row + i * dir.0 <= -1
                || row + i * dir.0 >= self.rows as isize
                || col + i * dir.1 <= -1
                || col + i * dir.1 >= self.cols as isize
            {
                return false;
            }
            match self.spots[(row + i * dir.0) as usize][(col + i * dir.1) as usize] {
                Spot::OccupiedSeat => return true,
                Spot::EmptySeat => return false,
                Spot::Floor => continue,
            }
        }

        false
    }

    /// Compute the number of visible occupied seats to the left, right, top, bottom,
    /// and on the diagonals starting from the seat located at `coords`.
    fn adjacent_visible(&self, coord: (usize, usize)) -> u32 {
        DIRECTIONS
            .iter()
            .map(|&dir| self.find_occupied_in_direction(coord, dir))
            .filter(|&f| f)
            .count() as u32
    }

    /// Update seat statuses and return the number of seats that have been changed
    fn update(&mut self, strategy: &Strategy, threshold: u32) -> bool {
        let mut changes = false;

        self.spots = self
            .spots
            .iter()
            .enumerate()
            .map(|(ri, r)| {
                r.iter()
                    .enumerate()
                    .map(|(ci, c)| {
                        let occ = match strategy {
                            Strategy::Adjacent => self.adjacent_occupied((ri, ci)),
                            Strategy::Visible => self.adjacent_visible((ri, ci)),
                        };
                        match c {
                            Spot::OccupiedSeat if occ >= threshold => {
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

    /// Count how many seats in the room are occupied
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
        assert_eq!(room.adjacent_visible((6, 4)), 0);
        assert_eq!(room.adjacent_visible((0, 0)), 1);
    }

    #[test]
    fn test_room_update() {
        let start = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let mut room: Room = start.into();

        let iteration_1 = "#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##";
        assert_eq!(&room._to_string(), start);
        room.update(&Strategy::Adjacent, 4);

        assert_eq!(&room._to_string(), iteration_1);
    }

    #[test]
    fn test_day_11_task_1() {
        let start = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(task_1_2(&start, &Strategy::Adjacent, 4), 37);
    }

    #[test]
    fn test_day_11_task_2() {
        let start = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(task_1_2(&start, &Strategy::Visible, 5), 26);
    }

    #[test]
    fn test_find_occupied_in_direction() {
        let iteration_1 = "#.L#.L#.L#\n#LLLLLL.LL\nL.L.L..#..\n##L#.#L.L#\nL.L#.LL.L#\n#.LLLL#.LL\n..#.L.....\nLLL###LLL#\n#.LLLLL#.L\n#.L#LL#.L#";
        let room: Room = iteration_1.into();
        assert_eq!(room.find_occupied_in_direction((2, 4), (0, 1)), true);
    }
}
