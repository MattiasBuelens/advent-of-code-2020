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

type TileFloor = HashSet<Vector2D>;

fn create_tile_floor(paths: &[Path]) -> TileFloor {
    let mut black_tiles = TileFloor::new();
    for path in paths {
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
    black_tiles
}

#[aoc(day24, part1)]
pub fn part1(paths: &[Path]) -> usize {
    create_tile_floor(paths).len()
}

fn get_neighbours(pos: Vector2D) -> [Vector2D; 6] {
    [
        pos + Direction::E.step(),
        pos + Direction::SE.step(),
        pos + Direction::SW.step(),
        pos + Direction::W.step(),
        pos + Direction::NW.step(),
        pos + Direction::NE.step(),
    ]
}

fn step(floor: &TileFloor) -> TileFloor {
    let min_x = floor.iter().map(|pos| pos.x).min().unwrap();
    let max_x = floor.iter().map(|pos| pos.x).max().unwrap();
    let min_y = floor.iter().map(|pos| pos.y).min().unwrap();
    let max_y = floor.iter().map(|pos| pos.y).max().unwrap();
    let mut new_floor = TileFloor::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let pos = Vector2D::new(x, y);
            let was_black = floor.contains(&pos);
            let black_neighbours = get_neighbours(pos)
                .iter()
                .filter(|neighbour| floor.contains(neighbour))
                .count();
            let is_black = match (was_black, black_neighbours) {
                (true, 0) => false,
                (true, count) if count > 2 => false,
                (false, 2) => true,
                _ => was_black,
            };
            if is_black {
                new_floor.insert(pos);
            }
        }
    }
    new_floor
}

#[aoc(day24, part2)]
pub fn part2(paths: &[Path]) -> usize {
    let mut floor = create_tile_floor(paths);
    for _i in 1..=100 {
        floor = step(&floor);
    }
    floor.len()
}
