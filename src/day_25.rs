//! This module contains the code
//! for the solution of the twenty-fifth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/25).

/// Compute the encryption key the handshake is trying to establish.
pub fn task_1(data: &[usize]) -> usize {
    transform(determine_loop_size(7, data[1]), data[0])
}

// Modular power
fn pow_mod(mut base: u64, mut exp: usize, modulus: u64) -> u64 {
    let mut acc = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            acc = acc * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    acc
}

/// Given the `subject_number` of the transformation and the `public_key` it should
/// generate, determine its loop size.
fn determine_loop_size(subject_number: u64, public_key: usize) -> usize {
    (1..)
        .into_iter()
        .find(|&l| pow_mod(subject_number, l, 20201227) == public_key as u64)
        .unwrap() as usize
}

/// The transformation used by the handshake.
fn transform(loop_size: usize, subject: usize) -> usize {
    (0..loop_size)
        .into_iter()
        .fold(1, |acc, _| (acc * subject) % 20201227)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_loop_size() {
        assert_eq!(determine_loop_size(7, 5764801), 8);
        assert_eq!(determine_loop_size(7, 17807724), 11);
    }

    #[test]
    fn test_day_25_task_1() {
        assert_eq!(task_1(&[5764801, 17807724]), 14897079);
    }
}
