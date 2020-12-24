use std::collections::HashSet;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    ///                 +y
    ///         NW     NE
    ///           \   /
    ///            \ /
    /// -x  W ------.-----E  +x
    ///            / \
    ///           /   \
    ///         SW     SE
    ///        -y
    ///
    ///  E = ( 1,  0)
    ///  W = (-1,  0)
    /// NE = ( 0,  1)
    /// SW = ( 0, -1)
    /// NW = (-1,  1) = W + NE
    /// SE = ( 1, -1) = E + SW
    fn step(&self) -> Vector2D {
        match self {
            Direction::E => Vector2D::new(1, 0),
            Direction::W => Vector2D::new(-1, 0),
            Direction::NE => Vector2D::new(0, 1),
            Direction::SW => Vector2D::new(0, -1),
            Direction::NW => Vector2D::new(-1, 1),
            Direction::SE => Vector2D::new(1, -1),
        }
    }
}

type Path = Vec<Direction>;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let mut path = vec![];
            while let Some(first) = chars.next() {
                path.push(match first {
                    'e' => Direction::E,
                    'w' => Direction::W,
                    's' => match chars.next() {
                        Some('e') => Direction::SE,
                        Some('w') => Direction::SW,
                        Some(second) => panic!("invalid direction: {}{}", first, second),
                        None => panic!("invalid direction: {}", first),
                    },
                    'n' => match chars.next() {
                        Some('e') => Direction::NE,
                        Some('w') => Direction::NW,
                        Some(second) => panic!("invalid direction: {}{}", first, second),
                        None => panic!("invalid direction: {}", first),
                    },
                    _ => panic!("invalid direction: {}", first),
                });
            }
            path
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Path]) -> usize {
    let mut black_tiles = HashSet::<Vector2D>::new();
    for path in input {
        let tile = path
            .iter()
            .fold(Vector2D::zero(), |pos, direction| pos + direction.step());
        if black_tiles.contains(&tile) {
            // Flip back to white
            black_tiles.remove(&tile);
        } else {
            // Flip to black
            black_tiles.insert(tile);
        }
    }
    black_tiles.len()
}

#[aoc(day24, part2)]
pub fn part2(input: &[Path]) -> i32 {
    todo!()
}
