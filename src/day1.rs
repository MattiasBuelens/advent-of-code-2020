#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|&x| get_fuel_part1(x)).sum()
}

fn get_fuel_part1(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

#[aoc(day1, part2)]
pub fn part2(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|&x| get_fuel_part2(x)).sum()
}

fn get_fuel_part2(mut mass: u32) -> u32 {
    let mut total_fuel = 0u32;
    while mass > 0 {
        let fuel = get_fuel_part1(mass);
        total_fuel += fuel;
        mass = fuel;
    }
    total_fuel
}
