//! This module contains the code
//! for the solution of the twenty-fourth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/24).
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Count the number of black tiles after all the flips from the
/// input have been performed.
pub fn task_1(data: &[String]) -> usize {
    data.iter()
        .map(|l| parse_directions(l))
        .fold(HashMap::new(), |mut cache, v| {
            *cache.entry(identify_tile(&v)).or_insert(0) += 1;
            cache
        })
        .into_iter()
        .filter(|(_, flips)| flips % 2 == 1)
        .count()
}

/// Compute the number of black tiles 100 days after the tiles have been
/// arranged according to the pattern specified in the input.
pub fn task_2(data: &[String]) -> usize {
    let mut floor: Floor = data
        .iter()
        .map(|l| parse_directions(l))
        .fold(HashMap::new(), |mut cache, v| {
            *cache.entry(identify_tile(&v)).or_insert(0) += 1;
            cache
        })
        .into_iter()
        .filter(|(_, flips)| flips % 2 == 1)
        .map(|(key, _)| key)
        .collect::<HashSet<_>>()
        .into();

    for _ in 0..100 {
        floor.flip();
    }

    floor.flipped.len()
}

/// All the directions that allow to reach one tile from the other.
#[derive(Debug, PartialEq)]
enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

/// Compute the tile coordinates from a set of directions.
fn identify_tile(directions: &Vec<Dir>) -> (isize, isize) {
    directions.iter().fold((0, 0), |pos, dir| match dir {
        Dir::E => (pos.0 + 2, pos.1),
        Dir::W => (pos.0 - 2, pos.1),
        Dir::NW => (pos.0 - 1, pos.1 + 1),
        Dir::NE => (pos.0 + 1, pos.1 + 1),
        Dir::SE => (pos.0 + 1, pos.1 - 1),
        Dir::SW => (pos.0 - 1, pos.1 - 1),
    })
}

/// A representation of the tiles on the floor (remembers which ones are black).
#[derive(Debug)]
struct Floor {
    flipped: HashSet<(isize, isize)>,
    x_lim: (isize, isize),
    y_lim: (isize, isize),
}

impl From<HashSet<(isize, isize)>> for Floor {
    fn from(flipped: HashSet<(isize, isize)>) -> Self {
        let x_lim = flipped
            .iter()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let y_lim = flipped
            .iter()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();

        Self {
            x_lim: (*x_lim.0, *x_lim.1),
            y_lim: (*y_lim.0, *y_lim.1),
            flipped,
        }
    }
}

impl Floor {
    /// Count the numbers of black tiles adjacent to `tile`.
    fn flipped_neighbors(&self, tile: (isize, isize)) -> usize {
        vec![(-1, 1), (1, 1), (-1, -1), (1, -1), (-2, 0), (2, 0)]
            .into_iter()
            .map(|(x, y)| (tile.0 + x, tile.1 + y))
            .filter(|tile| self.flipped.contains(&tile))
            .count()
    }

    /// Flip the tiles on the floor according to the rules stated in the problem description.
    fn flip(&mut self) {
        use std::cmp::{max, min};
        let mut set = HashSet::new();
        for x in self.x_lim.0 - 2..self.x_lim.1 + 3 {
            for y in self.y_lim.0 - 1..self.y_lim.1 + 2 {
                if y % 2 == 1 && x % 2 == 0 {
                    continue;
                }
                let flip_n = self.flipped_neighbors((x, y));
                let condition_1 = self.flipped.contains(&(x, y)) && !(flip_n == 0 || flip_n > 2);
                let condition_2 = !self.flipped.contains(&(x, y)) && flip_n == 2;
                if condition_1 || condition_2 {
                    set.insert((x, y));
                    self.x_lim = (min(self.x_lim.0, x), max(self.x_lim.1, x));
                    self.y_lim = (min(self.y_lim.0, y), max(self.y_lim.1, y));
                }
            }
        }
        self.flipped = set;
    }
}

/// Parse the directions from the string.
fn parse_directions(directions: &str) -> Vec<Dir> {
    directions
        .replace("nw", "x")
        .replace("ne", "y")
        .replace("se", "z")
        .replace("sw", "v")
        .chars()
        .map(|c| match c {
            'e' => Dir::E,
            'w' => Dir::W,
            'x' => Dir::NW,
            'y' => Dir::NE,
            'z' => Dir::SE,
            _ => Dir::SW,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parse_directions() {
        use Dir::*;
        let directions = parse_directions("wsenenewsewwnese");
        assert_eq!(directions, vec![W, SE, NE, NE, W, SE, W, W, NE, SE]);

        let directions = parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw");
        assert_eq!(
            directions,
            vec![NE, E, E, NE, SE, NW, NW, W, SW, NE, NE, W, NW, W, SE, W, NE, NW, SE, SW, E, SW]
        );
    }

    #[test]
    fn test_identify_tile() {
        let directions = parse_directions("wsenenewsewwnese");
        assert_eq!(identify_tile(&directions), (-2, 0));
    }

    #[test]
    fn test_day_24_task_1() {
        let input = [
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];

        let tile_ids = input
            .iter()
            .map(|d| identify_tile(&parse_directions(&d)))
            .collect::<HashSet<_>>();
        assert_eq!(tile_ids.len(), 15);
        assert_eq!(task_1(&input), 10);
    }

    #[test]
    fn test_floor() {
        let mut floor: Floor = vec![(2, 0), (1, 1)]
            .into_iter()
            .collect::<HashSet<_>>()
            .into();
        assert_eq!(floor.flipped.len(), 2);
        assert_eq!(floor.flipped_neighbors((0, 0)), 2);
        assert_eq!(floor.flipped_neighbors((2, 0)), 1);

        assert_eq!(floor.x_lim, (1, 2));
        assert_eq!(floor.y_lim, (0, 1));

        floor.flip();

        assert_eq!(floor.flipped.len(), 4);

        assert!(floor.flipped.contains(&(2, 0)));
        assert!(floor.flipped.contains(&(1, 1)));
        assert!(floor.flipped.contains(&(3, 1)));
        assert!(floor.flipped.contains(&(0, 0)));

        let input = [
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];

        let mut floor: Floor = input
            .iter()
            .map(|l| parse_directions(l))
            .fold(HashMap::new(), |mut cache, v| {
                *cache.entry(identify_tile(&v)).or_insert(0) += 1;
                cache
            })
            .into_iter()
            .filter(|(_, flips)| flips % 2 == 1)
            .map(|(key, _)| key)
            .collect::<HashSet<_>>()
            .into();

        assert_eq!(floor.flipped.len(), 10);

        for _ in 0..100 {
            floor.flip()
        }

        assert_eq!(floor.flipped.len(), 2208);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 15);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 12);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 25);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 14);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 23);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 28);

        // floor.flip();
        // assert_eq!(floor.flipped.len(), 41);
    }
}
