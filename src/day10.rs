#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_joltages(input: &[u64]) -> Vec<u64> {
    let mut joltages = input.to_vec();
    joltages.sort();

    // Treat the charging outlet near your seat as having an effective joltage rating of 0.
    joltages.insert(0, 0);

    // Your device has a built-in joltage adapter rated for 3 jolts higher
    // than the highest-rated adapter in your bag.
    joltages.push(joltages.last().unwrap() + 3);

    joltages
}

#[aoc(day10, part1)]
pub fn part1(input: &[u64]) -> usize {
    let differences = get_joltages(input)
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();

    let count_1 = differences.iter().filter(|&&x| x == 1).count();
    let count_3 = differences.iter().filter(|&&x| x == 3).count();
    count_1 * count_3
}

#[aoc(day10, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let joltages = get_joltages(input);
    let output_joltage = *joltages.last().unwrap();

    // ways_to_reach[joltage] returns the number of ways to reach joltage
    let mut ways_to_reach = vec![0u64; (output_joltage + 1) as usize];
    ways_to_reach.insert(0, 1); // charging outlet
    for joltage in joltages.into_iter().skip(1) {
        ways_to_reach[joltage as usize] = (joltage.saturating_sub(3)..=joltage)
            .map(|prev| ways_to_reach.get(prev as usize).unwrap_or(&0))
            .sum();
    }

    ways_to_reach[output_joltage as usize]
}
