//! This module contains the code
//! for the solution of the twenty-fourth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/24).
use std::collections::HashMap;

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
}
