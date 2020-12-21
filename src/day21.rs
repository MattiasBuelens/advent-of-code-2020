use std::collections::HashSet;
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

#[aoc(day21, part1)]
pub fn part1(foods: &[Food]) -> usize {
    let mut all_ingredients = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().cloned())
        .collect::<HashSet<_>>();
    let mut all_allergens = foods
        .iter()
        .flat_map(|food| food.allergens.iter().cloned())
        .collect::<HashSet<_>>();

    let ingredients_without_allergens = all_ingredients
        .iter()
        .filter(|&ingredient| {
            // Check that there is a counterexample for all possible allergens when mapped to this ingredient.
            all_allergens.iter().all(|allergen| {
                // Find a counterexample, i.e. any food where this (ingredient, allergen) mapping
                // would be invalid. That is: if it contains the allergen, but not the ingredient.
                foods.iter().any(|food| {
                    food.allergens.contains(allergen) && !food.ingredients.contains(ingredient)
                })
            })
        })
        .cloned()
        .collect::<HashSet<_>>();

    let appearances = foods.iter().map(|food| {
        food.ingredients.intersection(&ingredients_without_allergens).count()
    }).sum();

    appearances
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> i32 {
    todo!()
}
