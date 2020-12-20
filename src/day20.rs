use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

use crate::util::Vector2D;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    grid: [[bool; 10]; 10],
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|char| char == '#')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Tile { grid })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().try_for_each(|row| {
            row.iter()
                .try_for_each(|&cell| write!(f, "{}", if cell { '#' } else { '.' }))?;
            writeln!(f)
        })
    }
}

type Input = Vec<(u32, Tile)>;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|s| {
            let (id_line, tile_lines) = s.split_at(s.find('\n').unwrap());
            let id = id_line
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let tile = tile_lines[1..].parse().unwrap();
            (id, tile)
        })
        .collect()
}

impl Tile {
    fn border_top(&self) -> [bool; 10] {
        self.grid[0]
    }

    fn border_bottom(&self) -> [bool; 10] {
        self.grid[9]
    }

    fn border_left(&self) -> [bool; 10] {
        let mut border = [false; 10];
        for y in 0..10 {
            border[y] = self.grid[y][0];
        }
        border
    }

    fn border_right(&self) -> [bool; 10] {
        let mut border = [false; 10];
        for y in 0..10 {
            border[y] = self.grid[y][9];
        }
        border
    }

    fn flip_vertical(&self) -> Self {
        let mut tile = self.clone();
        tile.grid.reverse();
        tile
    }

    fn flip_horizontal(&self) -> Self {
        let mut tile = self.clone();
        for y in 0..10 {
            tile.grid[y].reverse()
        }
        tile
    }

    fn rotate_left(&self) -> Self {
        let mut tile = Self::default();
        for y in 0..10 {
            for x in 0..10 {
                tile.grid[y][x] = self.grid[x][9 - y];
            }
        }
        tile
    }

    fn rotate_right(&self) -> Self {
        let mut tile = Self::default();
        for y in 0..10 {
            for x in 0..10 {
                tile.grid[y][x] = self.grid[9 - x][y];
            }
        }
        tile
    }

    fn permutations(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotate_right(),
            self.rotate_right().rotate_right(),
            self.rotate_left(),
            self.flip_vertical(),
            self.flip_vertical().rotate_right(),
            self.flip_vertical().rotate_right().rotate_right(),
            self.flip_vertical().rotate_left(),
        ]
    }
}

fn solve(
    picture: &HashMap<Vector2D, (u32, Tile)>,
    size: i32,
    pos: Vector2D,
    remaining: HashMap<u32, Tile>,
) -> Option<HashMap<Vector2D, (u32, Tile)>> {
    let expected_top = picture
        .get(&(pos - Vector2D::new(0, 1)))
        .map(|(_, tile)| tile.border_bottom());
    let expected_left = picture
        .get(&(pos - Vector2D::new(1, 0)))
        .map(|(_, tile)| tile.border_right());
    for (id, tile) in &remaining {
        for perm in tile.permutations() {
            if let Some(expected_top) = expected_top {
                if perm.border_top() != expected_top {
                    continue;
                }
            }
            if let Some(expected_left) = expected_left {
                if perm.border_left() != expected_left {
                    continue;
                }
            }

            let mut picture = picture.clone();
            picture.insert(pos, (*id, perm));

            let mut remaining = remaining.clone();
            remaining.remove(id);
            if remaining.is_empty() {
                return Some(picture);
            }

            let mut pos = pos + Vector2D::new(1, 0);
            if pos.x == size {
                pos.x = 0;
                pos.y += 1;
            }
            if let Some(solution) = solve(&picture, size, pos, remaining) {
                return Some(solution);
            }
        }
    }
    None
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> u64 {
    let size = (input.len() as f32).sqrt() as i32;
    let solution = solve(
        &HashMap::new(),
        size,
        Vector2D::zero(),
        HashMap::from_iter(input.clone()),
    );
    let solution = solution.unwrap();

    vec![
        solution.get(&Vector2D::new(0, 0)),
        solution.get(&Vector2D::new(size - 1, 0)),
        solution.get(&Vector2D::new(0, size - 1)),
        solution.get(&Vector2D::new(size - 1, size - 1)),
    ]
    .iter()
    .map(|x| x.unwrap().0 as u64)
    .product()
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> u64 {
    todo!()
}
