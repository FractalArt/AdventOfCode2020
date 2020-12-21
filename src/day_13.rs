//! This module contains the code
//! for the solution of the thirteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/13).

/// Compute the product of bus number and waiting time for the
/// bus that minimizes waiting time.
pub fn task_1(data: &str) -> usize {
    let split = data.split_whitespace().collect::<Vec<_>>();
    assert_eq!(split.len(), 2);

    let time: usize = split[0].parse().unwrap();
    let result = split[1]
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|x| (x, (time / x) * x + if time % x != 0 { x } else { 0 } - time))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    result.0 * result.1
}

/// Find the earliest timestamp at which the first bus departs in such a way that
/// each subsequent bus departs on the subsequent minute.
pub fn task_2(data: &str) -> i128 {
    let split = data.split_whitespace().collect::<Vec<_>>();
    assert_eq!(split.len(), 2);
    split[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| match s.parse::<i128>() {
            Err(_) => None,
            Ok(val) => Some((modulus(-(i as i128), val), val)),
        })
        .fold((0, 1), chinese_remainder)
        .0
}

/// Chinese remainder theorem to combine two finite field values in a given cardinality.
/// The finite field values `c1` and `c2` are the represented as a tuple where the first
/// entry represents the finite field value while the second element denotes the cardinality
/// of the finite field.
/// NOTE: We assume that `c1.1` and `c2.1` are co-prime, which is the case since all
/// numbers in the input are primes.
fn chinese_remainder(c1: (i128, i128), c2: (i128, i128)) -> (i128, i128) {
    let m1 = mod_inv(c2.1, c1.1) * c2.1;
    let m2 = mod_inv(c1.1, c2.1) * c1.1;
    (modulus(m1 * c1.0 + m2 * c2.0, c1.1 * c2.1), c1.1 * c2.1)
}

/// Compute the modulus (corresponding to python's `%` operator).
fn modulus(n: i128, p: i128) -> i128 {
    ((n % p) + p) % p
}

/// Compute the modular inverse of an integer `n` modulo `p`
/// using the extended euclidean algorithm.
#[allow(clippy::many_single_char_names)]
fn mod_inv(n: i128, p: i128) -> i128 {
    let mut s = (1, 0);
    let mut t = (0, 1);
    let mut r = (p, n);

    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    t.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_task_1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(task_1(&input), 295);
    }

    #[test]
    fn test_modulus() {
        assert_eq!(modulus(-7, 2), 1);
        assert_eq!(modulus(6, 2), 0);
        assert_eq!(modulus(5, 2), 1);
        assert_eq!(modulus(-4, 2), 0);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(12, 8), 1);
        assert_eq!(mod_inv(4, 7), 2);
        assert_eq!(mod_inv(9, 13), 3);

        // Unit tests generated with Python
        assert_eq!(mod_inv(1973066479553, 41), 1);
        assert_eq!(mod_inv(17, 80895725661673), 9517144195491);
        assert_eq!(mod_inv(80895725661673, 17), -2);
    }

    #[test]
    fn test_chinese_remainder() {
        assert_eq!(chinese_remainder((0, 3), (4, 5)), (9, 15));
        assert_eq!(chinese_remainder((9, 15), (0, 11)), (99, 15 * 11));
        assert_eq!(chinese_remainder((9, 15), (0, 1)), (9, 15));

        // Unit test generated with Python
        assert_eq!(
            chinese_remainder((1250064264, 4577880463), (381, 431)),
            (56184629820, 1973066479553)
        );
        assert_eq!(
            chinese_remainder((56184629820, 1973066479553), (22, 41)),
            (53328979577751, 80895725661673)
        );
        assert_eq!(
            chinese_remainder((53328979577751, 80895725661673), (1, 17)),
            (538703333547789, 1375227336248441)
        );
    }

    #[test]
    fn test_day_13_task_2() {
        assert_eq!(task_2("939\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(task_2("1\n17,x,13,19"), 3417);
        assert_eq!(task_2("1\n67,7,59,61"), 754018);
        assert_eq!(task_2("1\n67,x,7,59,61"), 779210);
        assert_eq!(task_2("1\n67,7,x,59,61"), 1261476);
        assert_eq!(task_2("1\n1789,37,47,1889"), 1202161486);
    }
}
