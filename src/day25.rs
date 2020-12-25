#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn transform_step(value: u64, subject_number: u64) -> u64 {
    // Must use 64-bit for this multiplication!
    (value * subject_number) % 20201227
}

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _i in 0..loop_size {
        value = transform_step(value, subject_number);
    }
    value
}

fn crack_loop_size(key: u64, subject_number: u64) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    while value != key {
        value = transform_step(value, subject_number);
        loop_size += 1;
    }
    loop_size
}

#[aoc(day25, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let card_key = input[0];
    let door_key = input[1];

    let card_loop_size = crack_loop_size(card_key, 7);
    let door_loop_size = crack_loop_size(door_key, 7);
    assert_eq!(transform(7, card_loop_size), card_key);
    assert_eq!(transform(7, door_loop_size), door_key);

    let card_encryption_key = transform(door_key, card_loop_size);
    let door_encryption_key = transform(card_key, door_loop_size);
    assert_eq!(card_encryption_key, door_encryption_key);

    card_encryption_key
}
