//! This module contains the code
//! for the solution of the twenty-first day's challenges.
//!
//! The problem formulation for these challenges can
//! be found [here](https://adventofcode.com/2020/day/21).
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type INGREDIENTS<'a> = HashSet<&'a str>;
type ALLERGENS<'a> = Vec<&'a str>;
type RECIPES<'a> = Vec<(ALLERGENS<'a>, INGREDIENTS<'a>)>;

/// Count the number of times ingredients not containing allergens
/// appear in recipes.
pub fn task_1(data: &[String]) -> usize {
    let recipes: RECIPES = data.iter().map(|r| parse_recipe(r)).collect();
    let identified = get_ingredients_with_allergens(&recipes);
    let identified_ingredients = identified.values().collect::<HashSet<_>>();

    recipes
        .iter()
        .map(|(_, v)| v)
        .flat_map(|v| v.iter())
        .filter(|&&x| !identified_ingredients.contains(&x))
        .count()
}

/// Identify the ingredients containing allergens, sort them according to their allergens and
/// print the names of these ingredients in the sorted order separated by a comma.
pub fn task_2(data: &[String]) -> String {
    let recipes: RECIPES = data.iter().map(|r| parse_recipe(r)).collect();
    let identified = get_ingredients_with_allergens(&recipes);

    identified
        .iter()
        .sorted_by(|(allergen_1, _), (allergen_2, _)| Ord::cmp(&allergen_1, &allergen_2))
        .map(|(_, ingredient)| ingredient)
        .join(",")
}

/// Get a map relating the allergens (keys) with the ingredients they are contained in (values).
fn get_ingredients_with_allergens<'a>(recipes: &RECIPES<'a>) -> HashMap<&'a str, &'a str> {
    let allergens: HashSet<&&str> = recipes
        .iter()
        .map(|(a, _)| a)
        .flat_map(|v| v.into_iter())
        .collect();

    let mut identified = HashMap::new();
    let mut identified_ingredients = HashSet::new();

    while identified_ingredients.len() < allergens.len() {
        // For each allergen (that has not yet been identified), collect all ingredients
        // (which are not yet identified) in which it could be contained.
        let candidates = allergens
            .iter()
            .filter(|&&a| !identified.contains_key(a))
            .find_map(|&a| {
                // TODO: Use fold_first / reduce here once it becomes stable
                let mut candidates = recipes
                    .iter()
                    .filter(|(key, _)| key.as_slice().contains(&a));

                let first_set = candidates.next().unwrap().1.to_owned();
                let candidates = candidates
                    .fold(first_set, |acc, (_, ings)| {
                        acc.intersection(ings).cloned().collect::<HashSet<_>>()
                    })
                    .into_iter()
                    .filter(|&v| !identified_ingredients.contains(&v))
                    .collect::<Vec<_>>();

                if candidates.len() == 1 {
                    Some((a, candidates[0]))
                } else {
                    None
                }
            })
            .unwrap();

        identified.insert(*candidates.0, candidates.1);
        identified_ingredients.insert(candidates.1);
    }
    identified
}

/// Parse the recipe and return the allergens and ingredients in a tuple.
fn parse_recipe<'a>(data: &'a str) -> (ALLERGENS, INGREDIENTS) {
    lazy_static::lazy_static! {
        static ref REGEX: regex::Regex = regex::Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    }
    let captures = REGEX.captures(data).unwrap();
    (
        captures.get(2).unwrap().as_str().split(", ").collect(),
        captures
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_recipe() {
        let (allergens, ingredients) = parse_recipe(
            "tplp shptr krh tfn ztrgb crsp ghvj hzr rpf rhjbx kjfnqg (contains peanuts, dairy)",
        );

        // Check whether content counts match
        assert_eq!(allergens.len(), 2);
        assert_eq!(ingredients.len(), 11);

        // Precise inspection of the ingredients contents.
        assert!(ingredients.contains("tplp"));
        assert!(ingredients.contains("shptr"));
        assert!(ingredients.contains("krh"));
        assert!(ingredients.contains("tfn"));
        assert!(ingredients.contains("ztrgb"));
        assert!(ingredients.contains("crsp"));
        assert!(ingredients.contains("ghvj"));
        assert!(ingredients.contains("hzr"));
        assert!(ingredients.contains("rpf"));
        assert!(ingredients.contains("rhjbx"));
        assert!(ingredients.contains("kjfnqg"));
        assert!(!ingredients.contains("aoc2020"));

        // Precise inspection of the allergens contents
        assert!(allergens.as_slice().contains(&"peanuts"));
        assert!(allergens.as_slice().contains(&"dairy"));
    }

    #[test]
    fn test_day_21_task_1() {
        let input = [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!(task_1(&input), 5);
    }

    #[test]
    fn test_day_21_task_2() {
        let input = [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!(task_2(&input), "mxmxvkd,sqjhc,fvjkl");
    }
}
