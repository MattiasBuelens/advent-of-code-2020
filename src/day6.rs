use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect())
                .collect()
        })
        .collect::<Vec<_>>()
}

#[aoc(day6, part1)]
pub fn part1(groups: &[Vec<Vec<char>>]) -> usize {
    let group_counts = groups.iter().map(|group| {
        let answers = group.iter().flatten().collect::<HashSet<_>>();
        answers.len()
    });
    group_counts.sum()
}

#[aoc(day6, part2)]
pub fn part2(groups: &[Vec<Vec<char>>]) -> usize {
    let group_counts = groups.iter().map(|group| {
        let answers = group
            .iter()
            .map(|person| person.iter().cloned().collect::<HashSet<char>>())
            .fold(None, |left: Option<HashSet<char>>, right| {
                Some(match left {
                    None => right,
                    Some(left) => left.intersection(&right).cloned().collect(),
                })
            })
            .unwrap();
        answers.len()
    });
    group_counts.sum()
}
