#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut numbers = input.to_vec();
    let limit = 2020;
    for i in numbers.len()..limit {
        let (&last, rest) = numbers.split_last().unwrap();
        let prev = rest.iter().rposition(|&x| x == last);
        numbers.push(match prev {
            Some(prev) => ((i - 1) - prev) as i32,
            None => 0,
        });
    }
    *numbers.last().unwrap()
}

#[aoc(day15, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}
