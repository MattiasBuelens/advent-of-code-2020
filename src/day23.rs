use std::collections::HashMap;

struct Cup {
    label: u32,
    prev_label: u32,
    next_label: u32,
}

struct Cups {
    head_label: u32,
    cups_by_label: HashMap<u32, Cup>,
}

impl Cups {
    fn from_iter<I: IntoIterator<Item = u32>>(x: I) -> Self {
        let mut it = x.into_iter();
        let mut cups_by_label = HashMap::new();

        // Head
        let head_label = it.next().unwrap();
        let mut cup = Cup {
            label: head_label.clone(),
            prev_label: 0,
            next_label: 0,
        };
        while let Some(label) = it.next() {
            let prev_label = cup.label;
            // Connect previous cup to new cup
            cup.next_label = label;
            // Insert previous cup
            cups_by_label.insert(cup.label.clone(), cup);
            // Create new cup
            cup = Cup {
                label,
                prev_label,
                next_label: 0,
            }
        }

        // Connect head to tail
        cups_by_label.get_mut(&head_label).unwrap().prev_label = cup.label;
        // Connect tail to head
        cup.next_label = head_label.clone();
        // Insert tail
        cups_by_label.insert(cup.label.clone(), cup);

        Cups {
            head_label,
            cups_by_label,
        }
    }

    fn len(&self) -> usize {
        self.cups_by_label.len()
    }

    fn get(&self, label: u32) -> &Cup {
        self.cups_by_label.get(&label).unwrap()
    }

    fn get_mut(&mut self, label: u32) -> &mut Cup {
        self.cups_by_label.get_mut(&label).unwrap()
    }

    fn head(&self) -> &Cup {
        self.get(self.head_label)
    }

    fn tail(&self) -> &Cup {
        self.get_prev(self.head())
    }

    fn advance_head(&mut self) {
        self.head_label = self.head().next_label;
    }

    fn replace_head(&mut self, label: u32) {
        self.head_label = label;
    }

    fn get_prev(&self, cup: &Cup) -> &Cup {
        self.cups_by_label.get(&cup.prev_label).unwrap()
    }

    fn get_next(&self, cup: &Cup) -> &Cup {
        self.cups_by_label.get(&cup.next_label).unwrap()
    }

    fn remove(&mut self, label: u32) {
        let cup = self.get(label);
        let prev_label = cup.prev_label;
        let next_label = cup.next_label;

        let prev = self.get_mut(prev_label);
        prev.next_label = next_label;

        let next = self.get_mut(next_label);
        next.prev_label = prev_label;

        if label == self.head_label {
            self.head_label = next_label;
        }
    }

    fn insert_after(&mut self, ref_label: u32, new_label: u32) {
        let ref_next = self.get(ref_label).next_label;

        let new_cup = self.get_mut(new_label);
        new_cup.prev_label = ref_label;
        new_cup.next_label = ref_next;

        let prev = self.get_mut(ref_label);
        prev.next_label = new_label;

        let next = self.get_mut(ref_next);
        next.prev_label = new_label;
    }

    fn into_vec(self) -> Vec<u32> {
        let mut result = vec![];
        let mut cup = self.head();
        loop {
            result.push(cup.label);
            cup = self.get_next(cup);
            if cup.label == self.head_label {
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
    let current_label = current.label;
    // Pick up three cups after the current cup
    let picked = {
        let mut picked = vec![];
        let mut next_cup_label = current.next_label;
        for _i in 0..3 {
            picked.push(next_cup_label);
            cups.remove(next_cup_label);
            next_cup_label = cups.get(next_cup_label).next_label;
        }
        picked
    };
    // Select the destination cup
    let destination_label = {
        // The crab selects a destination cup: the cup with a label equal
        // to the current cup's label minus one.
        let mut label = current_label - 1;
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
    for label in picked.into_iter().rev() {
        cups.insert_after(destination_label, label);
    }
    // The crab selects a new current cup: the cup which is immediately clockwise
    // of the current cup.
    cups.advance_head();
}

#[aoc(day23, part1)]
pub fn part1(input: &[u32]) -> String {
    let mut cups = Cups::from_iter(input.to_vec());
    // Play 100 rounds
    for _i in 1..=100 {
        play_round(&mut cups);
    }
    // Rotate cup with label 1 into first position
    cups.replace_head(1);
    // Drop first cup (with label 1)
    cups.remove(1);
    // Join without extra characters
    cups.into_vec()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[aoc(day23, part2)]
pub fn part2(input: &[u32]) -> i32 {
    todo!()
}
