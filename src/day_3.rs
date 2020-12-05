//! This module contains the code
//! for the solution of the third day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/3).

/// Compute the solution to task 1 of day 3.
///
/// It takes as input a vector of strings `data` in which
/// each line corresponds to on level of the terrain. The
/// remaining to variables describe the descent strategy across
/// the terrain, i.e. take `slope_x` steps to the right, followed
/// by taking `slope_y` steps down.
///
/// The number of trees encountered during the descent is returned.
pub fn task_1(data: &[String], slope_x: usize, slope_y: usize) -> u64 {
    let mut current_x: usize = 0;
    let mut current_y: usize = 0;
    let mut counter: u64 = 0;

    loop {
        current_x += slope_x;
        current_y += slope_y;
        if current_y >= data.len() {
            break counter;
        }

        if data[current_y].chars().cycle().nth(current_x).unwrap() == '#' {
            counter += 1;
        }
    }
}

/// The solution to task two.
///
/// Takes a vector of `slopes` and multiplies the number of trees
/// encountered in the descent corresponding to each slope.
///
/// The terrain with the empty spots and trees is stored in
/// the vector `data`, where each entry corresponds to one line
/// or equivalently one height level in the terrain.
///
/// The solution delegates the work to the function [`task_1`](`crate::day_3::task_1`).
pub fn task_2(data: &[String], slopes: &[(usize, usize)]) -> u64 {
    slopes
        .iter()
        .map(|(x, y)| task_1(data, *x, *y))
        .product::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3_task_1() {
        let terrain = &[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
        assert_eq!(task_1(terrain, 3, 1), 7);
    }

    #[test]
    fn test_day_3_task_2() {
        let terrain = &[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
        let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        assert_eq!(task_2(terrain, slopes), 336);
    }
}
