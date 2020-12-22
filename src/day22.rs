use std::collections::{HashSet, VecDeque};

type Input = (VecDeque<usize>, VecDeque<usize>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

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

fn play_round(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) {
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

fn play_game(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (Player, Vec<usize>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        play_round(&mut deck1, &mut deck2);
    }
    let (winner, mut winner_deck) = if deck1.is_empty() {
        (Player::Player2, deck2)
    } else {
        (Player::Player1, deck1)
    };
    (winner, winner_deck.make_contiguous().to_vec())
}

fn player_score(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * card)
        .sum()
}

#[aoc(day22, part1)]
pub fn part1(input: &Input) -> usize {
    let (deck1, deck2) = input.clone();
    let (_, winner_deck) = play_game(deck1, deck2);
    player_score(&winner_deck)
}

fn play_recursive_round(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) {
    let top1 = deck1.pop_front().unwrap();
    let top2 = deck2.pop_front().unwrap();
    let winner = if deck1.len() >= top1 && deck2.len() >= top2 {
        // Winner is determined by a sub-game
        let mut deck1 = deck1.clone();
        let mut deck2 = deck2.clone();
        deck1.truncate(top1);
        deck2.truncate(top2);
        let (winner, _) = play_recursive_game(deck1, deck2);
        winner
    } else {
        // Winner is the higher-value card
        if top1 > top2 {
            Player::Player1
        } else {
            Player::Player2
        }
    };
    if winner == Player::Player1 {
        deck1.push_back(top1);
        deck1.push_back(top2);
    } else {
        deck2.push_back(top2);
        deck2.push_back(top1);
    }
}

fn play_recursive_game(
    mut deck1: VecDeque<usize>,
    mut deck2: VecDeque<usize>,
) -> (Player, Vec<usize>) {
    let mut previous_decks = HashSet::<(VecDeque<usize>, VecDeque<usize>)>::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let current_decks = (deck1.clone(), deck2.clone());
        if previous_decks.contains(&current_decks) {
            break; // Recursion!
        } else {
            previous_decks.insert(current_decks);
        }
        play_recursive_round(&mut deck1, &mut deck2);
    }
    // Player 1 wins if player 2's deck is empty, or because of recursion.
    // Player 2 wins if player 1's deck is empty.
    let (winner, mut winner_deck) = if deck1.is_empty() {
        (Player::Player2, deck2)
    } else {
        (Player::Player1, deck1)
    };
    (winner, winner_deck.make_contiguous().to_vec())
}

#[aoc(day22, part2)]
pub fn part2(input: &Input) -> usize {
    let (deck1, deck2) = input.clone();
    let (_, winner_deck) = play_recursive_game(deck1, deck2);
    player_score(&winner_deck)
}
