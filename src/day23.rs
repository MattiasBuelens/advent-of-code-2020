use std::collections::VecDeque;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn play_round(cups: &mut VecDeque<u32>) {
    let largest_cup_label = cups.len() as u32;
    // The current cup is at the head of the queue
    let current = *cups.front().unwrap();
    // Pick up three cups after the current cup
    let picked = cups.drain(1..=3).collect::<Vec<_>>();
    // Select the destination cup
    let destination_label = {
        // The crab selects a destination cup: the cup with a label equal
        // to the current cup's label minus one.
        let mut label = current - 1;
        loop {
            if label == 0 {
                // If at any point in this process the value goes below the lowest value
                // on any cup's label, it wraps around to the highest value on any cup's label
                // instead.
                label = largest_cup_label;
            }
            if picked.contains(&label) {
                // If this would select one of the cups that was just picked up,
                // the crab will keep subtracting one until it finds a cup that wasn't
                // just picked up.
                label -= 1;
            } else {
                break label;
            }
        }
    };
    // The crab places the cups it just picked up so that they are immediately clockwise
    // of the destination cup.
    let destination_index = cups
        .iter()
        .position(|&cup| cup == destination_label)
        .unwrap();
    for cup in picked.into_iter().rev() {
        cups.insert(destination_index + 1, cup);
    }
    // The crab selects a new current cup: the cup which is immediately clockwise
    // of the current cup.
    cups.rotate_left(1);
}

#[aoc(day23, part1)]
pub fn part1(input: &[u32]) -> String {
    let mut cups = VecDeque::from(input.to_vec());
    // Play 100 rounds
    for _i in 1..=100 {
        play_round(&mut cups);
    }
    // Rotate cup with label 1 into first position
    let pos_1 = cups.iter().position(|&cup| cup == 1).unwrap();
    cups.rotate_left(pos_1);
    // Drop first cup (with label 1)
    cups.pop_front();
    // Join without extra characters
    cups.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[aoc(day23, part2)]
pub fn part2(input: &[u32]) -> i32 {
    todo!()
}
