use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
struct Cups {
    head: u32,
    next: HashMap<u32, u32>,
}

impl FromIterator<u32> for Cups {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut it = iter.into_iter();

        // Head
        let head = it.next().unwrap();
        let mut next = HashMap::new();

        let mut prev = head.clone();
        while let Some(current) = it.next() {
            next.insert(prev.clone(), current.clone());
            prev = current;
        }

        // Connect tail to head
        next.insert(prev, head.clone());

        Cups { head, next }
    }
}

impl Cups {
    fn len(&self) -> usize {
        self.next.len()
    }

    fn head(&self) -> u32 {
        self.head
    }

    fn set_head(&mut self, label: u32) {
        self.head = label;
    }

    fn advance_head(&mut self) {
        self.head = self.get_next(self.head);
    }

    fn get_next(&self, label: u32) -> u32 {
        *self.next.get(&label).unwrap()
    }

    fn remove(&mut self, prev_label: u32, label: u32) {
        assert_eq!(self.get_next(prev_label), label);
        let next_label = self.get_next(label);
        self.next.insert(prev_label, next_label);
        if label == self.head {
            self.head = next_label;
        }
    }

    fn insert_after(&mut self, ref_label: u32, new_label: u32) {
        let next_label = self.get_next(ref_label);
        self.next.insert(ref_label, new_label);
        self.next.insert(new_label, next_label);
    }

    fn into_vec(self) -> Vec<u32> {
        let mut result = vec![];
        let mut label = self.head;
        loop {
            result.push(label);
            label = self.get_next(label);
            if label == self.head {
                break;
            }
        }
        result
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn play_round(cups: &mut Cups) {
    let largest_cup_label = cups.len() as u32;
    // The current cup is at the head of the queue
    let current = cups.head();
    // Pick up three cups after the current cup
    let picked = {
        let mut picked = [0; 3];
        for i in 0..picked.len() {
            let label = cups.get_next(current);
            picked[i] = label;
            cups.remove(current, label);
        }
        picked
    };
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
    for &label in picked.iter().rev() {
        cups.insert_after(destination_label, label);
    }
    // The crab selects a new current cup: the cup which is immediately clockwise
    // of the current cup.
    cups.advance_head();
}

#[aoc(day23, part1)]
pub fn part1(input: &[u32]) -> String {
    let mut cups = input.to_vec().into_iter().collect::<Cups>();
    // Play 100 rounds
    for _i in 1..=100 {
        play_round(&mut cups);
    }
    // Rotate cup with label 1 into first position
    cups.set_head(1);
    cups.into_vec()
        .into_iter()
        .skip(1) // drop first cup (with label 1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("") // join without extra characters
}

#[aoc(day23, part2)]
pub fn part2(input: &[u32]) -> u64 {
    let largest_cup_label = input.len() as u32;
    let mut input = input.to_vec();
    input.extend((largest_cup_label + 1)..=1_000_000);

    let mut cups = input.into_iter().collect::<Cups>();
    for _i in 1..=10_000_000 {
        play_round(&mut cups);
    }

    // Multiply next two cups after cup with label 1
    let next1 = cups.get_next(1);
    let next2 = cups.get_next(next1);
    (next1 as u64) * (next2 as u64)
}
