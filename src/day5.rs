use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(parse_seat_id).collect()
}

fn parse_seat_id(seat: &str) -> i32 {
    i32::from_str_radix(
        &seat
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!("invalid input"),
            })
            .collect::<String>(),
        2,
    )
    .expect("invalid input")
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> i32 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let seats = input.into_iter().cloned().collect::<HashSet<i32>>();
    let min_seat = seats.iter().min().unwrap();
    let max_seat = seats.iter().max().unwrap();
    for seat in (min_seat + 1)..=(max_seat - 1) {
        if !seats.contains(&seat) && seats.contains(&(seat - 1)) && seats.contains(&(seat + 1)) {
            return seat;
        }
    }
    panic!("seat not found")
}
