//! This module contains the code
//! for the solution of the seventeenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/17).
use std::cmp::{max, min};
use std::collections::HashSet;

/// A type for 4D coordinates.
type XYZW = (isize, isize, isize, isize);

/// Compute the number of active cubes after 6 steps in a 3D simulation.
pub fn task_1(data: &str, cycles: usize) -> usize {
    let mut grid = Grid::new(data, false);
    for _ in 0..cycles {
        grid.update();
    }
    grid.actives.len()
}

/// Compute the number of active cubes after 6 steps in a 4D simulation.
pub fn task_2(data: &str, cycles: usize) -> usize {
    let mut grid = Grid::new(data, true);
    for _ in 0..cycles {
        grid.update();
    }
    grid.actives.len()
}

/// A grid that only tracks the active positions as well
/// as the minimum and maximal values on each axis (they
/// are the same on each axis).
struct Grid {
    actives: HashSet<XYZW>,
    min_index: isize,
    max_index: isize,
    four_dimensional: bool,
}

impl Grid {
    /// Create a new grid from the string representation of the initial state (puzzle input).
    fn new(data: &str, four_dimensional: bool) -> Self {
        let raw_grid = data
            .split_whitespace()
            .map(|l| {
                l.chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Make sure the grid is quadratic
        assert_eq!(raw_grid.len(), raw_grid[0].len());

        Self {
            four_dimensional,
            min_index: 0,
            max_index: raw_grid.len() as isize,
            actives: raw_grid
                .into_iter()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.into_iter().enumerate().filter_map(move |(x, b)| {
                        if b {
                            Some((x as isize, y as isize, 0, 0))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        }
    }

    /// Given a coordinate in the grid, return the number of active neighbors.
    fn active_neighbors(&self, coord: XYZW) -> usize {
        let mut sum = 0;
        // Check if we need to loop over the full fourth dimension or leave it at 0.
        let range_w = if !self.four_dimensional {
            0..1
        } else {
            max(coord.3 - 1, self.min_index)..min(coord.3 + 2, self.max_index)
        };
        for w in range_w {
            for z in max(coord.2 - 1, self.min_index)..min(coord.2 + 2, self.max_index) {
                for y in max(coord.1 - 1, self.min_index)..min(coord.1 + 2, self.max_index) {
                    for x in max(coord.0 - 1, self.min_index)..min(coord.0 + 2, self.max_index) {
                        if (x, y, z, w) == coord {
                            continue;
                        }
                        if self.actives.contains(&(x, y, z, w)) {
                            sum += 1
                        }
                    }
                }
            }
        }
        sum
    }

    /// Update the grid using the rules provided in the puzzle description.
    fn update(&mut self) {
        let mut new_actives = HashSet::new();
        let mut new_min: isize = self.min_index;
        let mut new_max: isize = self.max_index;
        // Check if we need to loop over the full fourth dimension or leave it at 0.
        // -1, +1 to try whether something changes in previously non-active spots
        let range_w = if !self.four_dimensional {
            0..1
        } else {
            self.min_index - 1..self.max_index + 1
        };
        for w in range_w {
            for z in self.min_index - 1..self.max_index + 1 {
                for y in self.min_index - 1..self.max_index + 1 {
                    for x in self.min_index - 1..self.max_index + 1 {
                        let active_neighbors = self.active_neighbors((x, y, z, w));
                        let condition_1 = self.actives.contains(&(x, y, z, w))
                            && (active_neighbors == 2 || active_neighbors == 3);
                        let condition_2 =
                            !self.actives.contains(&(x, y, z, w)) && active_neighbors == 3;
                        if condition_1 || condition_2 {
                            new_actives.insert((x, y, z, w));
                            let local_min = vec![x, y, z, w].into_iter().min().unwrap();
                            let local_max = vec![x, y, z, w].into_iter().max().unwrap();
                            if local_min < new_min || local_max > new_max {
                                new_min -= 1;
                                new_max += 1;
                            }
                        }
                    }
                }
            }
        }
        self.actives = new_actives;
        self.min_index = new_min;
        self.max_index = new_max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new(".#.\n..#\n###", false);

        // Test actives
        assert!(grid.actives.contains(&(1, 0, 0, 0)));
        assert!(grid.actives.contains(&(2, 1, 0, 0)));
        assert!(grid.actives.contains(&(0, 2, 0, 0)));
        assert!(grid.actives.contains(&(1, 2, 0, 0)));
        assert!(grid.actives.contains(&(2, 2, 0, 0)));
        assert!(!grid.actives.contains(&(0, 0, 0, 0)));

        // Test active neightbours
        assert_eq!(grid.active_neighbors((0, 0, 0, 0)), 1);
        assert_eq!(grid.active_neighbors((1, 2, 0, 0)), 3);
    }

    #[test]
    fn test_day_17_task_1() {
        let input = ".#.\n..#\n###";
        assert_eq!(task_1(&input, 3), 38);
    }

    #[test]
    fn test_day_17_task_2() {
        let input = ".#.\n..#\n###";
        assert_eq!(task_2(&input, 6), 848);
    }
}
