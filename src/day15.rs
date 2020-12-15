use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn solve(input: &[i32], limit: usize) -> i32 {
    let mut last_seen = HashMap::<i32, usize>::new();
    let (&last, rest) = input.split_last().unwrap();
    for (i, num) in rest.iter().enumerate() {
        last_seen.insert(*num, i);
    }
    let mut last = last;
    for i in input.len()..limit {
        let prev = last_seen.get(&last);
        let next = match prev {
            Some(prev) => ((i - 1) - prev) as i32,
            None => 0,
        };
        last_seen.insert(last, i - 1);
        last = next;
    }
    last
}

#[aoc(day15, part1)]
pub fn part1(input: &[i32]) -> i32 {
    solve(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &[i32]) -> i32 {
    solve(input, 30_000_000)
}
