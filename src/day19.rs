use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Rule {
    Single(char),
    Union(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('"') {
            Ok(Rule::Single(s.chars().next().unwrap()))
        } else {
            let options = s
                .split(" | ")
                .map(|option| option.split(' ').map(|id| id.parse().unwrap()).collect())
                .collect();
            Ok(Rule::Union(options))
        }
    }
}

type Input = (HashMap<usize, Rule>, Vec<String>);

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let id = parts.next().unwrap().parse().unwrap();
            let rule = parts.next().unwrap().parse().unwrap();
            (id, rule)
        })
        .collect();
    let messages = lines.map(|line| line.to_owned()).collect();
    (rules, messages)
}

fn match_rule_in_state<'a>(
    rule: &Rule,
    rules: &HashMap<usize, Rule>,
    state: &'a str,
) -> HashSet<&'a str> {
    match rule {
        Rule::Single(c) => state.strip_prefix(*c).into_iter().collect(),
        Rule::Union(options) => {
            // Try all of the options
            options
                .iter()
                .flat_map(|sequence| {
                    // Match all sub rules in sequence
                    sequence
                        .iter()
                        .fold(HashSet::from_iter(Some(state)), |states, sub_rule| {
                            match_rule(*sub_rule, rules, &states)
                        })
                })
                .collect()
        }
    }
}

fn match_rule<'a>(
    rule_id: usize,
    rules: &HashMap<usize, Rule>,
    states: &HashSet<&'a str>,
) -> HashSet<&'a str> {
    if states.is_empty() {
        return HashSet::new();
    }
    let rule = rules.get(&rule_id).unwrap();
    // Advance through all possible states simultaneously
    states
        .into_iter()
        .flat_map(|&state| match_rule_in_state(rule, rules, state))
        .collect()
}

fn match_rule_complete(rule_id: usize, rules: &HashMap<usize, Rule>, s: &str) -> bool {
    let final_states = match_rule(rule_id, rules, &HashSet::from_iter(Some(s)));
    // At least one match must have consumed the entire string
    final_states.contains(&"")
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    let (rules, messages) = input;
    messages
        .iter()
        .filter(|message| match_rule_complete(0, rules, message))
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> usize {
    let (rules, messages) = input;
    let mut rules = rules.clone();
    rules.insert(8, "42 | 42 8".parse().unwrap());
    rules.insert(11, "42 31 | 42 11 31".parse().unwrap());

    messages
        .iter()
        .filter(|message| match_rule_complete(0, &rules, message))
        .count()
}
