//! This module contains the code
//! for the solution of the twentieth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/20).
use ndarray::prelude::*;

pub fn task_1(data: &str, tile_size: usize) -> usize {
    let tiles = data
        .split("\n\n")
        .map(|t| Tile::from_string(t, tile_size))
        .collect::<Vec<_>>();

    let img_dim = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(img_dim.pow(2), tiles.len());

    let matches = tiles
        .iter()
        .map(|t1| {
            let mut set = std::collections::HashSet::new();

            for t2 in tiles.iter() {
                for s in t1.get_matching_sides(t2) {
                    set.insert(s);
                }
            }

            set
        })
        .map(|s| s.len())
        .collect::<Vec<_>>();
    println!("matches: {:#?}", matches);
    3
}

#[derive(Debug, PartialEq)]
struct Tile {
    id: usize,
    dim: usize,
    top: Vec<usize>,
    right: Vec<usize>,
    bottom: Vec<usize>,
    left: Vec<usize>,
}

impl Tile {
    // TODO: Get rid of the `ndarray` dependence
    fn from_string(data: &str, tile_dim: usize) -> Tile {
        let mut lines = data.split("\n");
        let id = lines
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut content: Array2<usize> = Array::zeros((tile_dim, tile_dim));
        for (row, line) in lines.enumerate() {
            for (col, c) in line.chars().enumerate() {
                content[[row, col]] = match c {
                    '#' => 1,
                    _ => 0,
                }
            }
        }

        Tile {
            id,
            dim: tile_dim,
            top: content.index_axis(Axis(0), 0).to_vec(),
            right: content.index_axis(Axis(1), tile_dim - 1).to_vec(),
            bottom: content.index_axis(Axis(0), tile_dim - 1).to_vec(),
            left: content.index_axis(Axis(1), 0).to_vec(),
        }
    }

    fn get_side(&self, index: usize) -> &Vec<usize> {
        match index {
            0 => &self.top,
            1 => &self.right,
            2 => &self.bottom,
            3 => &self.left,
            _ => panic!("Got invalid index"),
        }
    }

    fn get_matching_sides(&self, other: &Tile) -> Vec<usize> {
        let mut tmp = vec![];
        for i1 in 0..4 {
            for i2 in 0..4 {
                if self.get_side(i1).iter().eq(other.get_side(i2).iter()) {
                    tmp.push(i1);
                }

                if self.get_side(i1).iter().eq(other.get_side(i2).iter().rev()) {
                    tmp.push(i1);
                }
            }
        }
        tmp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_string() {
        let input = "Tile 1:\n#.\n##";
        let tile = Tile::from_string(&input, 2);
        assert_eq!(tile.id, 1);

        assert_eq!(tile.top[0], 1);
        assert_eq!(tile.top[1], 0);

        assert_eq!(tile.right[0], 0);
        assert_eq!(tile.right[1], 1);

        assert_eq!(tile.bottom[0], 1);
        assert_eq!(tile.bottom[1], 1);

        assert_eq!(tile.left[0], 1);
        assert_eq!(tile.left[1], 1);

        let input = "Tile 1:\n###\n...\n###";
        let tile = Tile::from_string(&input, 3);
        assert_eq!(tile.id, 1);

        assert_eq!(tile.top[0], 1);
        assert_eq!(tile.top[1], 1);
        assert_eq!(tile.top[2], 1);

        assert_eq!(tile.right[0], 1);
        assert_eq!(tile.right[1], 0);
        assert_eq!(tile.right[2], 1);

        assert_eq!(tile.bottom[0], 1);
        assert_eq!(tile.bottom[1], 1);
        assert_eq!(tile.bottom[2], 1);

        assert_eq!(tile.left[0], 1);
        assert_eq!(tile.left[1], 0);
        assert_eq!(tile.left[2], 1);
    }

    #[test]
    fn test_tile_matches() {
        let tile_1 = Tile::from_string("Tile 1:\n###\n#..\n...", 3);
        let tile_2 = Tile::from_string("Tile 2:\n..#\n.#.\n#.#", 3);
        let matches = tile_1.get_matching_sides(&tile_2);
        assert_eq!(matches.len(), 2);
    }
}
