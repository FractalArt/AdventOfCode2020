//! This module contains the code
//! for the solution of the second day challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/2).

/// Check whether a given password is valid.
///
/// For this to be case, the password `pwd` needs to
/// contain `character` at least `min_occurrence` times
/// and at most `max_occurrence` times.
fn is_valid_password_1(
    pwd: &str,
    character: char,
    min_occurrence: usize,
    max_occurrence: usize,
) -> bool {
    let count = pwd.matches(character).count();
    count >= min_occurrence && count <= max_occurrence
}

/// Check whether a given password is valid.
///
/// For this to be the case, the password `pwd` needs to contain
/// `character` in exactly one of the positions `pos_1` or  `pos_2`.
///
/// __Note__: Both `pos_1` and `pos_2` are 1-based. Hence `pos_1`
/// corresponds to index 0 in the vector.
fn is_valid_password_2(pwd: &str, character: char, pos_1: usize, pos_2: usize) -> bool {
    pwd.chars()
        .enumerate()
        .filter(|(i, _)| *i + 1 == pos_1 || *i + 1 == pos_2)
        .filter(|(_, c)| *c == character)
        .count()
        == 1
}

/// Process the input for the second day's challenge.
///
/// Each `input` consists of a single line from the
/// puzzle input file and it is given in the form
///
/// `1-3 a: passwd`
///
/// where `passwd` is the password, and `a` is the letter
/// that is allowed to occur from `1` up to `3` times in the password
/// for it to be considered valid.
///
/// The function parses the different components using regular expressions
/// and returns them in a tuple.
fn process_input_day_2(input: &str) -> (String, char, usize, usize) {
    // Avoid recompiling the regular expression.
    lazy_static::lazy_static! {
        static ref REGEX_D2: regex::Regex = regex::Regex::new(r"^(\d*)-(\d*) ([a-z]): ([a-z]*)$").unwrap();
    }
    let cap = REGEX_D2.captures(input).unwrap();
    let min: usize = cap[1].parse().unwrap();
    let max: usize = cap[2].parse().unwrap();
    let target: char = cap[3].parse().unwrap();
    let password: String = cap[4].parse().unwrap();
    (password, target, min, max)
}

/// Compute the solution of the second day's first challenge.
///
/// For each line in the vector `data`
/// check whether the password matches the given
/// criterion and return the number of valid passwords.
///
/// ### Example
///
/// ```
/// # use aoc2020::day_2::task_1;
/// assert_eq!(task_1(&vec![
///     "1-3 a: abcde".to_string(),
///     "1-3 b: cdefg".to_string(),
/// ]), 1);
/// ```
pub fn task_1(data: &Vec<String>) -> usize {
    data.iter()
        .map(|s| match process_input_day_2(&s) {
            (pwd, c, min, max) => is_valid_password_1(&pwd, c, min, max),
        })
        .filter(|b| *b)
        .count()
}

/// Compute the solution of the second day's second challenge.
///
/// For each line in the vector `data`
/// check whether the password matches the given
/// criterion and return the number of valid passwords.
///
/// ### Example
///
/// ```
/// # use aoc2020::day_2::task_2;
/// assert_eq!(task_2(&vec![
///     "1-3 a: abcde".to_string(),
///     "1-3 b: bdbfg".to_string(),
/// ]), 1);
/// ```
pub fn task_2(data: &Vec<String>) -> usize {
    data.iter()
        .map(|s| match process_input_day_2(&s) {
            (pwd, c, pos_1, pos_2) => is_valid_password_2(&pwd, c, pos_1, pos_2),
        })
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_task_1() {
        assert_eq!(
            task_1(&vec![
                "1-3 a: abcde".to_string(),
                "1-3 b: cdefg".to_string(),
                "2-9 c: ccccccccc".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_process_input_day_2() {
        assert_eq!(
            process_input_day_2("1-3 b: cdefg"),
            ("cdefg".to_string(), 'b', 1, 3)
        );
    }

    #[test]
    fn test_is_valid_password_1() {
        assert_eq!(is_valid_password_1("abcde", 'a', 1, 3), true);
        assert_eq!(is_valid_password_1("abcde", 'a', 2, 4), false);
    }

    #[test]
    fn test_is_valid_password_2() {
        assert_eq!(is_valid_password_2("abcde", 'a', 1, 3), true);
        assert_eq!(is_valid_password_2("cdefg", 'b', 1, 3), false);
        assert_eq!(is_valid_password_2("ccccccccc", 'b', 2, 3), false);
    }
}
