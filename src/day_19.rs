//! This module contains the code
//! for the solution of the nineteenth day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/19).
use itertools::Itertools;
use std::collections::HashMap;

/// Find all the images that match rule 0.
pub fn task_1(data: &str) -> usize {
    let split = data.split("\n\n").collect::<Vec<&str>>();

    let rules: HashMap<usize, Rule> = split[0].split("\n").map(|r| extract_rule(r)).collect();

    let matches = get_all_matches(0, &rules);

    split[1]
        .split_whitespace()
        .filter(|img| matches.iter().any(|m| img == m))
        .count()
}

/// Extract a rule from its string representation.
fn extract_rule<'a>(rule: &'a str) -> (usize, Rule<'a>) {
    lazy_static::lazy_static! {
        static ref RE_LETTERS: regex::Regex = regex::Regex::new(r#"^(\d*): "(\w)"$"#).unwrap();
        static ref RE_RULE_1: regex::Regex = regex::Regex::new(r"^(\d*):(( \d*)*) \|(( \d*)*)$").unwrap();
        static ref RE_RULE_2: regex::Regex = regex::Regex::new(r"^(\d*):(( \d*)*)$").unwrap();
    }

    if RE_LETTERS.is_match(rule) {
        let captures = RE_LETTERS.captures(rule).unwrap();
        (
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            Rule::Letter(captures.get(2).unwrap().as_str()),
        )
    } else if RE_RULE_1.is_match(rule) {
        let captures = RE_RULE_1.captures(rule).unwrap();
        (
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            Rule::Either(
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split_whitespace()
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect(),
                captures
                    .get(4)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split_whitespace()
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect(),
            ),
        )
    } else {
        let captures = RE_RULE_2.captures(rule).unwrap();
        (
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            Rule::One(
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split_whitespace()
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect(),
            ),
        )
    }
}

/// Get all the strings that match a given rule.
fn get_all_matches<'a>(rule: usize, rules: &HashMap<usize, Rule<'a>>) -> Vec<String> {
    match &rules[&rule] {
        Rule::Letter(a) => vec![a.to_string()],
        Rule::One(v) => v
            .iter()
            .map(|&r| get_all_matches(r, &rules))
            .multi_cartesian_product()
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<_>>(),
        Rule::Either(v1, v2) => v1
            .iter()
            .map(|&r| get_all_matches(r, &rules))
            .multi_cartesian_product()
            .map(|v| v.into_iter().collect::<String>())
            .chain(
                v2.iter()
                    .map(|&r| get_all_matches(r, &rules))
                    .multi_cartesian_product()
                    .map(|v| v.into_iter().collect::<String>()),
            )
            .collect(),
    }
}

/// The different combinations a rule can be.
#[derive(Debug, PartialEq)]
enum Rule<'a> {
    Letter(&'a str),
    One(Vec<usize>),
    Either(Vec<usize>, Vec<usize>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_regex() {
        let re1: regex::Regex = regex::Regex::new(r"^(\d*):(( \d*)*)$").unwrap();
        let re2: regex::Regex = regex::Regex::new(r"^(\d*):(( \d*)*) \|(( \d*)*)$").unwrap();

        let captures = re1.captures("115: 12").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "115");
        assert_eq!(captures.get(2).unwrap().as_str().trim(), "12");

        let captures = re1.captures("115: 12 13").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "115");
        assert_eq!(captures.get(2).unwrap().as_str().trim(), "12 13");

        let captures = re2.captures("115: 12 13 | 14 15").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "115");
        assert_eq!(captures.get(2).unwrap().as_str().trim(), "12 13");
        assert_eq!(captures.get(4).unwrap().as_str().trim(), "14 15");
    }

    #[test]
    fn test_letter_regex() {
        let letter_regex: regex::Regex = regex::Regex::new(r#"^(\d*): "\w"$"#).unwrap();

        assert!(letter_regex.is_match(r#"3: "c""#));
    }

    #[test]
    fn test_extract_rule() {
        assert_eq!(extract_rule(r#"3: "a""#), (3, Rule::Letter("a")));
        assert_eq!(extract_rule(r"4: 12"), (4, Rule::One(vec![12])));
        assert_eq!(extract_rule(r"4: 12 13"), (4, Rule::One(vec![12, 13])));
        assert_eq!(
            extract_rule(r"4: 12 13 | 14 15"),
            (4, Rule::Either(vec![12, 13], vec![14, 15]))
        );
        assert_eq!(
            extract_rule(r"4: 12 | 14 15"),
            (4, Rule::Either(vec![12], vec![14, 15]))
        );
        assert_eq!(
            extract_rule(r"4: 12 13 | 14"),
            (4, Rule::Either(vec![12, 13], vec![14]))
        );
        assert_eq!(
            extract_rule(r"4: 12 | 14"),
            (4, Rule::Either(vec![12], vec![14]))
        );
    }

    #[test]
    fn test_multi_cartesian_product() {
        let possibilities = vec![vec!["a", "b"], vec!["c"]];
        let combis = possibilities
            .into_iter()
            .multi_cartesian_product()
            .collect::<std::collections::HashSet<_>>();
        assert!(combis.contains(&vec!["a", "c"]));
        assert!(combis.contains(&vec!["b", "c"]));
    }

    #[test]
    fn test_get_all_matches() {
        let rules = vec![
            (0, Rule::One(vec![4, 1, 5])),
            (1, Rule::Either(vec![2, 3], vec![3, 2])),
            (2, Rule::Either(vec![4, 4], vec![5, 5])),
            (3, Rule::Either(vec![4, 5], vec![5, 4])),
            (4, Rule::Letter("a")),
            (5, Rule::Letter("b")),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let matches = get_all_matches(0, &rules);
        assert_eq!(matches.len(), 8);
        assert!(matches.as_slice().contains(&"aaaabb".to_string()));
        assert!(matches.as_slice().contains(&"aaabab".to_string()));
        assert!(matches.as_slice().contains(&"abbabb".to_string()));
        assert!(matches.as_slice().contains(&"abbbab".to_string()));
        assert!(matches.as_slice().contains(&"aabaab".to_string()));
        assert!(matches.as_slice().contains(&"aabbbb".to_string()));
        assert!(matches.as_slice().contains(&"abaaab".to_string()));
        assert!(matches.as_slice().contains(&"ababbb".to_string()));
    }

    #[test]
    fn test_day_19_task_1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        assert_eq!(task_1(&input), 2);
    }
}
