use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
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

fn match_rule<'a>(rule_id: usize, rules: &HashMap<usize, Rule>, s: &'a str) -> Option<&'a str> {
    let rule = rules.get(&rule_id).unwrap();
    match rule {
        Rule::Single(c) => s.strip_prefix(*c),
        Rule::Union(options) => {
            // Match any of the options
            'option_loop: for sequence in options {
                // Match all sub rules in sequence
                let mut s = s;
                for sub_rule in sequence {
                    if let Some(rest) = match_rule(*sub_rule, rules, s) {
                        s = rest;
                    } else {
                        // Sequence does not match
                        continue 'option_loop;
                    }
                }
                return Some(s);
            }
            // No option matched
            None
        }
    }
}

fn match_rule_complete(rule_id: usize, rules: &HashMap<usize, Rule>, s: &str) -> bool {
    let m = match_rule(rule_id, rules, s);
    matches!(m, Some(""))
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
pub fn part2(input: &Input) -> i32 {
    todo!()
}
