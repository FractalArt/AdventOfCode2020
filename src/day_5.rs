//! This module contains the code
//! for the solution of the fifth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/5).
use itertools::Itertools;

/// Determine the maximum seat number.
///
/// The input `data` consists of the lines of the input
/// file where each line contains the string representation
/// of a seat number.
///
/// After parsing these representations to integers, the maximal
/// seat number is returned.
pub fn task_1(data: &[String]) -> usize {
    data.iter().map(|l| extract_seat_number(l)).max().unwrap()
}

/// Determine our seat.
///
/// This is the missing seat. We need to ignore that at the beginning and at the
/// end, some seats are missing, albeit knowing that our direct seat neighboring
/// seat numbers are present.
///
/// The `input` data consists of the lines of the input file
/// where each line contains the string representation of a
/// seat number.
///
/// After parsing these representations to integers, they are sorted in order
/// to determine the missing seat.
pub fn task_2(data: &[String]) -> usize {
    let sorted = data
        .iter()
        .map(|seat| extract_seat_number(seat))
        .sorted()
        .collect::<Vec<_>>();

    let mut index: usize = 1;

    loop {
        if sorted[index - 1] + 1 != sorted[index] {
            break sorted[index - 1] + 1;
        }
        index += 1;
    }
}

/// Given the string representation of the `seat` determine the seat number.
fn extract_row_number(seat: &str) -> usize {
    usize::from_str_radix(
        &seat
            .chars()
            .take_while(|c| c == &'F' || c == &'B')
            .map(|c| if c == 'F' { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

/// Given the string representation of the `seat` determine its row.
fn extract_col_number(seat: &str) -> usize {
    usize::from_str_radix(
        &seat
            .chars()
            .skip_while(|c| c == &'F' || c == &'B')
            .map(|c| if c == 'L' { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

/// Given the string representation of the `seat` determine its column.
fn extract_seat_number(seat: &str) -> usize {
    extract_row_number(seat) * 8 + extract_col_number(seat)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_row_number() {
        assert_eq!(extract_row_number("BFFFBBFRRR"), 70);
        assert_eq!(extract_row_number("FFFBBBFRRR"), 14);
        assert_eq!(extract_row_number("BBFFBBFRLL"), 102);
    }

    #[test]
    fn test_extract_col_number() {
        assert_eq!(extract_col_number("BFFFBBFRRR"), 7);
        assert_eq!(extract_col_number("FFFBBBFRRR"), 7);
        assert_eq!(extract_col_number("BBFFBBFRLL"), 4);
    }
    #[test]
    fn test_extract_seat_number() {
        assert_eq!(extract_seat_number("BFFFBBFRRR"), 567);
        assert_eq!(extract_seat_number("FFFBBBFRRR"), 119);
        assert_eq!(extract_seat_number("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_day_5_task_1() {
        let input = [
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];
        assert_eq!(task_1(&input), 820);
    }
}
