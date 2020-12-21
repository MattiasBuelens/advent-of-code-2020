use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_owned())
            .collect();
        let allergens = parts
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .map(|x| x.to_owned())
            .collect();
        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Food> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn is_impossible_mapping(foods: &[Food], ingredient: &String, allergen: &String) -> bool {
    // Find a counterexample, i.e. any food where this (ingredient, allergen) mapping
    // would be invalid. That is: if it contains the allergen, but not the ingredient.
    foods
        .iter()
        .any(|food| food.allergens.contains(allergen) && !food.ingredients.contains(ingredient))
}

fn get_ingredients_without_allergens(
    foods: &[Food],
    all_ingredients: &HashSet<String>,
    all_allergens: &HashSet<String>,
) -> HashSet<String> {
    all_ingredients
        .iter()
        .filter(|&ingredient| {
            // Check that there is a counterexample for all possible allergens when mapped to this ingredient.
            all_allergens
                .iter()
                .all(|allergen| is_impossible_mapping(foods, ingredient, allergen))
        })
        .cloned()
        .collect::<HashSet<_>>()
}

#[aoc(day21, part1)]
pub fn part1(foods: &[Food]) -> usize {
    let all_ingredients = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().cloned())
        .collect::<HashSet<_>>();
    let all_allergens = foods
        .iter()
        .flat_map(|food| food.allergens.iter().cloned())
        .collect::<HashSet<_>>();

    let ingredients_without_allergens =
        get_ingredients_without_allergens(foods, &all_ingredients, &all_allergens);

    let appearances = foods
        .iter()
        .map(|food| {
            food.ingredients
                .intersection(&ingredients_without_allergens)
                .count()
        })
        .sum();

    appearances
}

fn solve(
    foods: &[Food],
    mapping: HashMap<String, String>,
    unknown_ingredients: &HashSet<String>,
    unknown_allergens: &HashSet<String>,
) -> Option<HashMap<String, String>> {
    if unknown_ingredients.is_empty() {
        return Some(mapping);
    }
    for ingredient in unknown_ingredients {
        for allergen in unknown_allergens {
            if is_impossible_mapping(foods, ingredient, allergen) {
                continue;
            }
            let mut mapping = mapping.clone();
            let mut unknown_ingredients = unknown_ingredients.clone();
            let mut unknown_allergens = unknown_allergens.clone();
            mapping.insert(ingredient.clone(), allergen.clone());
            unknown_ingredients.remove(ingredient);
            unknown_allergens.remove(allergen);
            if let Some(solution) = solve(foods, mapping, &unknown_ingredients, &unknown_allergens)
            {
                return Some(solution);
            }
        }
    }
    None
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> String {
    let all_ingredients = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().cloned())
        .collect::<HashSet<_>>();
    let all_allergens = foods
        .iter()
        .flat_map(|food| food.allergens.iter().cloned())
        .collect::<HashSet<_>>();

    let ingredients_without_allergens =
        get_ingredients_without_allergens(foods, &all_ingredients, &all_allergens);

    let unknown_ingredients = all_ingredients
        .difference(&ingredients_without_allergens)
        .cloned()
        .collect::<HashSet<_>>();
    let unknown_allergens = all_allergens;
    assert_eq!(unknown_ingredients.len(), unknown_allergens.len());

    let mut solution = solve(
        foods,
        HashMap::new(),
        &unknown_ingredients,
        &unknown_allergens,
    )
    .unwrap()
    .into_iter()
    .collect::<Vec<_>>();

    solution.sort_by_key(|(_, allergen)| allergen.clone());
    solution
        .into_iter()
        .map(|(ingredient, _)| ingredient)
        .collect::<Vec<_>>()
        .join(",")
}
