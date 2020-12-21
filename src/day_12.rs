//! This module contains the code
//! for the solution of the twelfth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/12).
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w)(\d*)$").unwrap();
}

/// Compute the Manhattan distance of the ship after performing
/// all the navigation actions provided in `data`. In this task
/// the ship is rotated.
pub fn task_1(data: &[String]) -> isize {
    let position = data
        .iter()
        .map(|l| extract_action(&l))
        .fold(Ship::new(), |mut ship, action| {
            ship.apply_action(action);
            ship
        });

    position.north_south.abs() + position.east_west.abs()
}

/// Compute the Manhattan distance of the ship after performing
/// all the navigation actions provided in `data`. In this task
/// the waypoint is rotated.
pub fn task_2(data: &[String]) -> isize {
    let position =
        data.iter()
            .map(|l| extract_action(&l))
            .fold(WayPoint::new(), |mut waypoint, action| {
                waypoint.apply_action(action);
                waypoint
            });

    position.ship_position.0.abs() + position.ship_position.1.abs()
}

/// Extract the action from its string representation in the input file.
fn extract_action(string: &str) -> Action {
    let captures = RE.captures(string).unwrap();
    let amount = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
    match captures.get(1).unwrap().as_str() {
        "N" => Action::N(amount),
        "S" => Action::S(amount),
        "E" => Action::E(amount),
        "W" => Action::W(amount),
        "L" => Action::L(amount),
        "R" => Action::R(amount),
        _ => Action::F(amount),
    }
}

/// The direction in which the ship can face.
#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    /// Rotate the ship by a given number of degrees.
    /// It is assumed that the degrees are a multiple of 90 degrees,
    /// where a rotation to the right is positive and to the left negative.
    fn rotate(&self, degrees: isize) -> Self {
        assert_eq!(degrees % 90, 0);
        let direction = self.clone() as isize;
        let degrees_int = (degrees / 90) % 4;
        let mut new_direction = (direction + degrees_int) % 4;
        if new_direction < 0 {
            new_direction += 4
        }
        match new_direction {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            _ => Self::West,
        }
    }
}

/// All of the actions that can be read from the input.
#[derive(Debug, PartialEq)]
enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

/// A ship is characterized by the direction it is currently facing in,
/// its `north_south` position (north is positive, south negative) as
/// well as its `east_west` position (east positive, west negative).
struct Ship {
    facing: Direction,
    north_south: isize,
    east_west: isize,
}

impl Ship {
    /// Create a new ship, which is facing `East` as mentioned in the challenge.
    fn new() -> Self {
        Self {
            facing: Direction::East,
            north_south: 0,
            east_west: 0,
        }
    }

    /// Apply a given `action` from the input file.
    fn apply_action(&mut self, action: Action) {
        match action {
            Action::F(val) => match self.facing {
                Direction::East => self.east_west += val,
                Direction::West => self.east_west -= val,
                Direction::North => self.north_south += val,
                Direction::South => self.north_south -= val,
            },
            Action::N(val) => self.north_south += val,
            Action::S(val) => self.north_south -= val,
            Action::E(val) => self.east_west += val,
            Action::W(val) => self.east_west -= val,
            Action::R(val) => self.facing = self.facing.rotate(val),
            Action::L(val) => self.facing = self.facing.rotate(-val),
        }
    }
}

/// A waypoint for Part 2.
struct WayPoint {
    ship_position: (isize, isize),
    north_south: isize,
    east_west: isize,
}

impl WayPoint {
    /// Create a waypoint with the initial conditions as defined in the challenge.
    fn new() -> Self {
        Self {
            ship_position: (0, 0),
            north_south: 1,
            east_west: 10,
        }
    }

    /// Apply a given action to the waypoint.
    fn apply_action(&mut self, action: Action) {
        match action {
            Action::N(val) => self.north_south += val,
            Action::S(val) => self.north_south -= val,
            Action::E(val) => self.east_west += val,
            Action::W(val) => self.east_west -= val,
            Action::F(val) => {
                self.ship_position = (
                    self.ship_position.0 + val * self.north_south,
                    self.ship_position.1 + val * self.east_west,
                )
            }
            Action::L(val) => self.rotate(-val % 360),
            Action::R(val) => self.rotate(val % 360),
        }
    }

    /// Rotate the waypoint.
    fn rotate(&mut self, angle: isize) {
        assert!(angle % 90 == 0);
        for _ in 0..angle.abs() / 90 {
            let (new_north_south, new_east_west) = (
                -angle.signum() * self.east_west,
                angle.signum() * self.north_south,
            );
            self.north_south = new_north_south;
            self.east_west = new_east_west;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_action() {
        assert_eq!(extract_action("N4"), Action::N(4));
        assert_eq!(extract_action("S4"), Action::S(4));
        assert_eq!(extract_action("E5"), Action::E(5));
        assert_eq!(extract_action("W3"), Action::W(3));
        assert_eq!(extract_action("L180"), Action::L(180));
        assert_eq!(extract_action("R90"), Action::R(90));
        assert_eq!(extract_action("R90"), Action::R(90));
        assert_eq!(extract_action("F96"), Action::F(96));
    }

    #[test]
    fn test_direction_rotate() {
        let n = Direction::North;
        let s = Direction::South;
        let e = Direction::East;
        let w = Direction::West;

        assert_eq!(n.rotate(360), n);
        assert_eq!(n.rotate(90), Direction::East);
        assert_eq!(n.rotate(180), Direction::South);
        assert_eq!(n.rotate(270), Direction::West);
        assert_eq!(n.rotate(-270), Direction::East);

        assert_eq!(s.rotate(180), Direction::North);
        assert_eq!(s.rotate(-180), Direction::North);

        assert_eq!(e.rotate(450), Direction::South);
        assert_eq!(w.rotate(-450), Direction::South);
    }

    #[test]
    fn test_rotate_waypoint() {
        let mut wp = WayPoint::new();
        assert_eq!(wp.north_south, 1);
        assert_eq!(wp.east_west, 10);

        wp.apply_action(Action::W(7));

        assert_eq!(wp.north_south, 1);
        assert_eq!(wp.east_west, 3);

        wp.apply_action(Action::N(1));

        assert_eq!(wp.north_south, 2);
        assert_eq!(wp.east_west, 3);

        // Rotate clockwise
        wp.rotate(180);
        assert_eq!(wp.north_south, -2);
        assert_eq!(wp.east_west, -3);

        wp.rotate(90);
        assert_eq!(wp.north_south, 3);
        assert_eq!(wp.east_west, -2);

        wp.rotate(90);
        assert_eq!(wp.north_south, 2);
        assert_eq!(wp.east_west, 3);

        // Now, we are back at the start.
        // Rotate counter-clockwise
        wp.rotate(-90);
        assert_eq!(wp.north_south, 3);
        assert_eq!(wp.east_west, -2);
    }

    #[test]
    fn test_waypoint() {
        let mut wp = WayPoint::new();

        wp.apply_action(Action::F(10));
        assert_eq!(wp.ship_position.0, 10);
        assert_eq!(wp.ship_position.1, 100);
        assert_eq!(wp.north_south, 1);
        assert_eq!(wp.east_west, 10);

        wp.apply_action(Action::N(3));
        assert_eq!(wp.ship_position.0, 10);
        assert_eq!(wp.ship_position.1, 100);
        assert_eq!(wp.north_south, 4);
        assert_eq!(wp.east_west, 10);

        wp.apply_action(Action::F(7));
        assert_eq!(wp.ship_position.0, 38);
        assert_eq!(wp.ship_position.1, 170);
        assert_eq!(wp.north_south, 4);
        assert_eq!(wp.east_west, 10);

        wp.apply_action(Action::R(90));
        assert_eq!(wp.ship_position.0, 38);
        assert_eq!(wp.ship_position.1, 170);
        assert_eq!(wp.north_south, -10);
        assert_eq!(wp.east_west, 4);

        wp.apply_action(Action::F(11));
        assert_eq!(wp.north_south, -10);
        assert_eq!(wp.east_west, 4);
        assert_eq!(wp.ship_position.0, -72);
        assert_eq!(wp.ship_position.1, 214);
    }

    #[test]
    fn test_day_12_task_1() {
        let input = [
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];
        assert_eq!(task_1(&input), 25);
    }

    #[test]
    fn test_day_12_task_2() {
        let input = [
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ];
        assert_eq!(task_2(&input), 286);
    }
}
