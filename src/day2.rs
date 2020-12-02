use std::str::FromStr;

#[derive(Debug)]
pub struct Policy {
    letter: char,
    left: usize,
    right: usize,
}

impl Policy {
    pub fn matches_part1(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();
        self.left <= count && count <= self.right
    }

    pub fn matches_part2(&self, password: &str) -> bool {
        let chars = password.chars().collect::<Vec<_>>();
        let left = chars[self.left - 1];
        let right = chars[self.right - 1];
        (left == self.letter) != (right == self.letter)
    }
}

impl FromStr for Policy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let mut bounds = parts.next().unwrap().split("-");
        Ok(Policy {
            left: bounds.next().unwrap().parse().unwrap(),
            right: bounds.next().unwrap().parse().unwrap(),
            letter: parts.next().unwrap().chars().next().unwrap(),
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Policy, String)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().to_owned(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| policy.matches_part1(password))
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| policy.matches_part2(password))
        .count()
}
