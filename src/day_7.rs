//! This module contains the code
//! for the solution of the seventh day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/7).
use regex::Regex;
use std::collections::{HashMap, HashSet};

type BagSet<'a> = HashSet<&'a str>;
type BagCounts<'a> = HashMap<&'a str, u32>;
type BagContentMap<'a> = HashMap<&'a str, BagCounts<'a>>;

/// Determine the number of colors top-level bags can have if they contain a shiny gold bag.
pub fn task_1(data: &[String]) -> usize {
    // Map listing each bag color, and the number of bags of a given color it can contain
    let content_rules: BagContentMap = data.iter().map(|l| extract_color_contents(l)).collect();

    content_rules
        .iter()
        .filter(|(_, content)| content.contains_key(&"shiny gold"))
        .fold(BagSet::new(), |mut acc, (&bag, _)| {
            for parent in get_parents(bag, &content_rules) {
                acc.insert(parent);
            }
            acc
        })
        .len()
}

/// Compute the number of bags contained in a bag of color `shiny gold`.
pub fn task_2(data: &[String]) -> u32 {
    // Extract the rules from the input
    let content_rules: BagContentMap = data.iter().map(|l| extract_color_contents(l)).collect();
    count_contained_bags("shiny gold", &content_rules)
}

/// Compute the parents of each bag containing a shiny gold bag at some level.
/// Notice that the bag under consideration is itself considered a possible
/// top-level bag.
fn get_parents<'a>(bag: &'a str, bcm: &BagContentMap<'a>) -> BagSet<'a> {
    let mut parent_set: BagSet = vec![bag].into_iter().collect();
    for candidate in bcm.keys() {
        if bcm[candidate].contains_key(&bag) {
            for parent in get_parents(candidate, &bcm) {
                parent_set.insert(&parent);
            }
        }
    }
    parent_set

    // The pure iterator solution is here below, by I think the
    // procedural code above looks a lot cleaner here.

    // bcm.keys().filter(|&key| bcm[key].contains_key(&bag)).fold(
    //     vec![bag].into_iter().collect::<HashSet<_>>(),
    //     |parents, key| {
    //         get_parents(key, &bcm)
    //             .into_iter()
    //             .fold(parents, |mut acc, p| {
    //                 acc.insert(p);
    //                 acc
    //             })
    //     },
    // )
}

/// Extract the color of a parent bag and the number and color of its content bags.
///
/// The input `rule` is a line from the input file, which formulates the rule.
fn extract_color_contents<'a>(rule: &'a str) -> (&str, BagCounts<'a>) {
    lazy_static::lazy_static! {
        static ref OVERALL: Regex = Regex::new(r"(\w* \w*) bags contain (.*).").unwrap();
    }
    lazy_static::lazy_static! {
        static ref NESTED: Regex = Regex::new(r"(\d*) (\w* \w*) bag").unwrap();
    }

    let captures = OVERALL.captures(rule).unwrap();
    let color: &str = &captures.get(1).unwrap().as_str();
    let content: HashMap<&str, u32> = captures
        .get(2)
        .unwrap()
        .as_str()
        .split(',')
        .filter_map(|x| match NESTED.captures(x) {
            Some(value) => Some((
                value.get(2).unwrap().as_str(),
                value.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            )),
            None => None,
        })
        .collect::<BagCounts>();

    (color, content)
}

/// Count the number of bags contained in `bag`.
fn count_contained_bags<'a>(bag: &'a str, content_map: &BagContentMap<'a>) -> u32 {
    match content_map[bag]
        .iter()
        .map(|x| x)
        .map(|(key, &count)| {
            let bags = count_contained_bags(key, &content_map);
            // If the bag has no content, we just count its number of occurrences
            if bags == 1 {
                count
            // If the bag has content, we count itself as well as its content
            } else {
                count * (1 + bags)
            }
        })
        .sum()
    {
        // If this bag has no contained bags, we just count the bag itself
        0 => 1,
        val => val,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_color_contents() {
        assert_eq!(
            extract_color_contents("vibrant magenta bags contain 2 dark lime bags."),
            (
                "vibrant magenta",
                vec![("dark lime", 2)].into_iter().collect()
            )
        );

        assert_eq!(
            extract_color_contents("dull white bags contain no other bags."),
            ("dull white", vec![].into_iter().collect())
        );

        assert_eq!(
            extract_color_contents("muted bronze bags contain 5 bright tomato bags, 5 light red bags, 2 shiny yellow bags, 2 dim teal bags."),
            (
                "muted bronze",
                vec![("bright tomato", 5), ("light red", 5), ("shiny yellow", 2), ("dim teal", 2)].into_iter().collect()
            )
        );
    }

    #[test]
    fn test_day_7_task_1() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        assert_eq!(task_1(&input), 4);
    }

    #[test]
    fn test_count_contained_bags() {
        let input_1 = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];

        let input_2 = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let content_rules_1: BagContentMap =
            input_1.iter().map(|l| extract_color_contents(l)).collect();

        let content_rules_2: BagContentMap =
            input_2.iter().map(|l| extract_color_contents(l)).collect();

        assert_eq!(count_contained_bags("dark blue", &content_rules_1), 2);
        assert_eq!(count_contained_bags("shiny gold", &content_rules_1), 126);
        assert_eq!(count_contained_bags("shiny gold", &content_rules_2), 32);
    }
}
