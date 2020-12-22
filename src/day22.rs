use std::collections::VecDeque;

type Input = (VecDeque<u32>, VecDeque<u32>);

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    assert_eq!(lines.next().unwrap(), "Player 1:");
    let deck1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(lines.next().unwrap(), "Player 2:");
    let deck2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    (deck1, deck2)
}

fn play_round(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) {
    let top1 = deck1.pop_front().unwrap();
    let top2 = deck2.pop_front().unwrap();
    if top1 > top2 {
        deck1.push_back(top1);
        deck1.push_back(top2);
    } else {
        deck2.push_back(top2);
        deck2.push_back(top1);
    }
}

fn player_score(deck: &[u32]) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i as u32 + 1) * card)
        .sum()
}

#[aoc(day22, part1)]
pub fn part1(input: &Input) -> u32 {
    let (mut deck1, mut deck2) = input.clone();
    while !deck1.is_empty() && !deck2.is_empty() {
        play_round(&mut deck1, &mut deck2);
    }
    let mut winner = if deck1.is_empty() { deck2 } else { deck1 };
    player_score(winner.make_contiguous())
}

#[aoc(day22, part2)]
pub fn part2(input: &Input) -> i32 {
    todo!()
}
