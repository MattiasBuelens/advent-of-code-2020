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
pub fn part1(foods: &[Food]) -> i32 {
    todo!()
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> i32 {
    todo!()
}
