use std::collections::LinkedList;
use std::iter::FromIterator;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn play_round(cups: &mut LinkedList<u32>) {
    let largest_cup_label = cups.len() as u32;
    let mut cursor = cups.cursor_front_mut();
    // The current cup is at the head of the queue
    let current = cursor.remove_current().unwrap();
    // Pick up three cups after the current cup
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    let picked = cursor.split_before();
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
    // FIXME THIS IS THE SLOW PART!!!
    while *cursor.current().unwrap() != destination_label {
        cursor.move_next();
    }
    cursor.splice_after(picked);
    // The crab selects a new current cup: the cup which is immediately clockwise
    // of the current cup.
    cups.push_back(current);
}

#[aoc(day23, part1)]
pub fn part1(input: &[u32]) -> String {
    let mut cups = LinkedList::from_iter(input.to_vec());
    // Play 100 rounds
    for _i in 1..=100 {
        play_round(&mut cups);
    }
    // Rotate cup with label 1 into first position
    let mut cursor = cups.cursor_front_mut();
    while *cursor.current().unwrap() != 1 {
        cursor.move_next();
    }
    let mut head = cursor.split_before();
    cups.append(&mut head);
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
