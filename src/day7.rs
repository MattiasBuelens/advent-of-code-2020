use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Rule {
    color: String,
    contents: Vec<(usize, String)>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" contain ");
            let color = parts
                .next()
                .unwrap()
                .strip_suffix(" bags")
                .unwrap()
                .to_owned();
            let contents = match parts.next().unwrap().strip_suffix(".").unwrap() {
                "no other bags" => vec![],
                s => s
                    .split(", ")
                    .map(|amount_color_bags| {
                        let (amount, color_bags) =
                            amount_color_bags.split_at(amount_color_bags.find(" ").unwrap());
                        let amount = amount.parse::<usize>().unwrap();
                        let (color, _) = color_bags.trim().split_at(color_bags.rfind(" ").unwrap());
                        let color = color.trim().to_owned();
                        (amount, color)
                    })
                    .collect::<Vec<_>>(),
            };
            Rule { color, contents }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(rules: &[Rule]) -> usize {
    let mut result = HashSet::<String>::new();
    loop {
        let new_bags = rules
            .iter()
            .filter_map(|rule| {
                if !result.contains(&rule.color)
                    && rule
                        .contents
                        .iter()
                        .any(|(_, bag)| bag == "shiny gold" || result.contains(bag))
                {
                    Some(rule.color.clone())
                } else {
                    None
                }
            })
            .collect::<HashSet<String>>();
        if new_bags.is_empty() {
            break;
        }
        result.extend(new_bags);
    }
    result.len()
}

#[aoc(day7, part2)]
pub fn part2(rules: &[Rule]) -> usize {
    let mut cache = HashMap::<String, usize>::new();
    let count = count_bags(&"shiny gold".to_owned(), &rules, &mut cache);
    count - 1 // minus the outer bag itself
}

fn count_bags(color: &String, rules: &[Rule], cache: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = cache.get(color) {
        return count;
    }
    let rule = rules
        .iter()
        .find(|rule| &rule.color == color)
        .expect("no rule for color");
    let count = rule
        .contents
        .iter()
        .map(|(amount, color)| amount * count_bags(color, rules, cache))
        .sum::<usize>()
        + 1;
    cache.insert(color.clone(), count);
    count
}
