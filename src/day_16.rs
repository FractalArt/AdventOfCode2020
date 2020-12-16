//! This module contains the code
//! for the solution of the sixteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/16).
use std::collections::HashMap;
use std::collections::HashSet;

/// Compute the sum of all invalid numbers in nearby tickets.
pub fn task_1(data: &[String]) -> usize {
    let valid_numbers: HashSet<usize> = collect_fields(data)
        .iter()
        .flat_map(|f| {
            (f.range_1.0..=f.range_1.1)
                .into_iter()
                .chain(f.range_2.0..=f.range_2.1)
        })
        .collect();

    collect_nearby_tickets(&data)
        .iter()
        .flat_map(|v| v.iter())
        .filter_map(|n| {
            if !valid_numbers.contains(n) {
                Some(*n)
            } else {
                None
            }
        })
        .sum()
}

/// Compute the sum of all six departure values on your ticket.
pub fn task_2(data: &[String]) -> usize {
    // Read the fields on the tickets
    let fields = collect_fields(&data);

    // Get a set of all valid numbers.
    let valid_numbers: HashSet<usize> = fields
        .iter()
        .flat_map(|f| {
            (f.range_1.0..=f.range_1.1)
                .into_iter()
                .chain(f.range_2.0..=f.range_2.1)
        })
        .collect();
        
    // Filter the nearby tickets, retaining only the valid ones.
    let valid_nearby = collect_nearby_tickets(&data)
        .into_iter()
        .filter(|t| t.iter().all(|v| valid_numbers.contains(v)))
        .collect::<Vec<_>>();
    
    // Get the content of your ticket.
    let your_ticket = collect_your_ticket(&data);
    
    // For each field, find the columns whose entries fullfil the field's ranges.
    let field_index_candidates = fields
        .iter()
        .map(|f| {
            let mut matches = vec![];
            for i in 0..valid_nearby[0].len() {
                if valid_nearby.iter().all(|v| {
                    v[i] >= f.range_1.0 && v[i] <= f.range_1.1
                        || v[i] >= f.range_2.0 && v[i] <= f.range_2.1
                }) {
                    matches.push(i);
                }
            }
            (&f.name, matches)
        })
        .collect::<HashMap<_, _>>();
    
    // By elimination, find the unambiguous columns associated to each field.
    let mut identified_cols = HashSet::new();
    let mut identified_fields = HashSet::new();
    let mut target_indices = vec![];
    while identified_cols.len() < fields.len() {
        for (f, m) in &field_index_candidates {
            if identified_fields.contains(&f) {
                continue;
            }
            let count = m
                .iter()
                .filter(|v| !identified_cols.contains(*v))
                .collect::<Vec<_>>();
            if count.len() != 1 {
                continue;
            } else {
                identified_fields.insert(f);
                identified_cols.insert(count[0]);
                if f.starts_with("departure") {
                    target_indices.push(count[0]);
                }
            }
        }
    }

    target_indices.iter().map(|&&v| your_ticket[v]).product()
}

/// Field with name and two ranges of valid numbers.
#[derive(Debug)]
struct Field {
    name: String,
    range_1: (usize, usize),
    range_2: (usize, usize),
}

/// Collect all the possible fields using the input data.
fn collect_fields(data: &[String]) -> Vec<Field> {
    lazy_static::lazy_static! {
        static ref RE_FIELD: regex::Regex = regex::Regex::new(r"^(\w*\s*\w*): (\d*)-(\d*) or (\d*)-(\d*)$").unwrap();
    }
    data.iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let captures = RE_FIELD.captures(l).unwrap();
            Field {
                name: captures.get(1).unwrap().as_str().to_string(),
                range_1: (
                    captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                range_2: (
                    captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                ),
            }
        })
        .collect()
}

/// Collect the information on your ticket using the input data.
fn collect_your_ticket(data: &[String]) -> Vec<usize> {
    let position = data.iter().position(|l| l == &"your ticket:").unwrap();
    data[position + 1]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

/// Collect the numbers stored in the nearby tickets.
fn collect_nearby_tickets(data: &[String]) -> Vec<Vec<usize>> {
    let position = data.iter().position(|l| l == &"nearby tickets:").unwrap();
    data.iter()
        .skip(position + 1)
        .map(|l| l.split(',').map(|c| c.parse::<usize>().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_fields() {
        let input = [
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        let fields = collect_fields(&input);
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "class".to_string());
        assert_eq!(fields[0].range_1, (1, 3));
        assert_eq!(fields[0].range_2, (5, 7));
        assert_eq!(fields[1].name, "row".to_string());
        assert_eq!(fields[2].name, "seat".to_string());
    }

    #[test]
    fn test_collect_your_ticket() {
        let input = [
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        assert_eq!(collect_your_ticket(&input), vec![7, 1, 14]);
    }

    #[test]
    fn test_collect_nearby_tickets() {
        let input = [
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        let nearby = collect_nearby_tickets(&input);
        assert_eq!(nearby[0], vec![7, 3, 47]);
        assert_eq!(nearby[1], vec![40, 4, 50]);
        assert_eq!(nearby[2], vec![55, 2, 20]);
        assert_eq!(nearby[3], vec![38, 6, 12]);
        assert_eq!(nearby.len(), 4);
    }

    #[test]
    fn test_day_16_task_1() {
        let input = [
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];
        assert_eq!(task_1(&input), 71);
    }
}
