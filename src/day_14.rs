//! This module contains the code
//! for the solution of the fourteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/14).
use itertools::Itertools;
use std::collections::HashMap;

/// Compute the sum of the entries present in the memory after all writing operations.
pub fn task_1(data: &[String]) -> usize {
    data.iter()
        .map(|l| parse_input(&l))
        .fold(("", HashMap::new()), |(mask, mut memory), val| match val {
            Input::Mask(s) => (s, memory),
            Input::Memory((index, val)) => {
                let entry = usize::from_str_radix(
                    &format!("{:0>36b}", val)
                        .chars()
                        .zip(mask.chars())
                        .map(|(c, mask)| if mask == 'X' { c } else { mask })
                        .collect::<String>(),
                    2,
                )
                .unwrap();

                memory.insert(index, entry);
                (mask, memory)
            }
        })
        .1
        .values()
        .sum::<usize>()
}

/// Compute the sum of the entries present in the memory after all writing operations.
pub fn task_2(data: &[String]) -> usize {
    data.iter()
        .map(|l| parse_input(&l))
        .fold(("", HashMap::new()), |(mask, mut memory), op| match op {
            Input::Mask(new_mask) => (new_mask, memory),
            Input::Memory((address, val)) => {
                for add in get_addresses(mask, address) {
                    memory.insert(add, val);
                }
                (mask, memory)
            }
        })
        .1
        .values()
        .sum::<usize>()
}

/// Parse an input line either to a mask or a memory access.
fn parse_input(line: &str) -> Input {
    lazy_static::lazy_static! {
        static ref RE_MASK: regex::Regex = regex::Regex::new(r"mask = (.*)$").unwrap();
        static ref RE_MEM: regex::Regex = regex::Regex::new(r"mem\[(\d*)\] = (\d*)$").unwrap();
    }
    if RE_MASK.is_match(line) {
        Input::Mask(RE_MASK.captures(line).unwrap().get(1).unwrap().as_str())
    } else {
        let captures = RE_MEM.captures(line).unwrap();
        Input::Memory((
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        ))
    }
}

/// Get all the addresses that can be obtained from a mask and the initially passes address.
#[allow(clippy::needless_collect)]
fn get_addresses(mask: &str, address: usize) -> Vec<usize> {
    // Collect the indices of the floating point bits
    let floating_bits = mask
        .chars()
        .enumerate()
        .filter_map(|(i, val)| if val == 'X' { Some(i) } else { None })
        .collect::<Vec<_>>();
    // Reference address where all `X` are set to 0
    let reference = mask
        .chars()
        .zip(format!("{:0>36b}", address).chars())
        .map(|(m, c)| match m {
            'X' => '0',
            '0' => c,
            _ => '1',
        })
        .collect::<Vec<_>>();

    // Loop over all length of combinations of floating indices
    (0..=floating_bits.len())
        // For all the combinations of that length
        .flat_map(|number| {
            // Create vector with entries corresponding to indices in the combination set to 1
            floating_bits
                .iter()
                .combinations(number)
                .map(|combination| {
                    let mut clone = reference.clone();
                    combination.into_iter().for_each(|i| clone[*i] = '1');
                    usize::from_str_radix(&clone.into_iter().collect::<String>(), 2).unwrap()
                })
        })
        .collect::<Vec<_>>()
}

/// Enumerate the possible inputs, they either designate
/// a mask or a memory writing operation.
#[derive(Debug, PartialEq)]
enum Input<'a> {
    /// Read a new mask.
    Mask(&'a str),
    /// Memory access, first entry is the location, second entry
    /// is the value to be written.
    Memory((usize, usize)),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("mask = 01X10101X11X01XX01X000011X1000110110"),
            Input::Mask("01X10101X11X01XX01X000011X1000110110")
        );
        assert_eq!(
            parse_input("mem[30135] = 4799584"),
            Input::Memory((30135, 4799584))
        );
    }

    #[test]
    fn test_bytes_representation() {
        assert_eq!(
            &format!("{:0>36b}", 11),
            "000000000000000000000000000000001011"
        );
    }

    #[test]
    fn test_day_14_task_1() {
        let input = [
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ];
        assert_eq!(task_1(&input), 165);
    }

    #[test]
    fn test_get_addresses() {
        let mask = "000000000000000000000000000000X1001X";
        let address = 42;
        let addresses = get_addresses(mask, address);
        assert_eq!(addresses.len(), 4);
        assert!(addresses.iter().find(|&&addr| addr == 26usize).is_some());
        assert!(addresses.iter().find(|&&addr| addr == 27usize).is_some());
        assert!(addresses.iter().find(|&&addr| addr == 58usize).is_some());
        assert!(addresses.iter().find(|&&addr| addr == 59usize).is_some());
        assert!(addresses.iter().find(|&&addr| addr == 60usize).is_none());
    }

    #[test]
    fn test_day_14_task_2() {
        let input = [
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ];
        assert_eq!(task_2(&input), 208);
    }
}
