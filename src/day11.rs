use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Floor,
    Empty,
    Occupied,
}

pub type Grid = HashMap<Vector2D, Tile>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                let tile = match char {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    _ => panic!("invalid tile"),
                };
                (Vector2D::new(x as i32, y as i32), tile)
            })
        })
        .collect()
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    let width = grid.keys().map(|pos| pos.x).max().unwrap();
    let height = grid.keys().map(|pos| pos.y).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            print!(
                "{}",
                match grid.get(&Vector2D::new(x, y)).unwrap() {
                    Tile::Floor => '.',
                    Tile::Empty => 'L',
                    Tile::Occupied => '#',
                }
            );
        }
        println!();
    }
}

fn part1_step(grid: &Grid) -> Grid {
    grid.iter()
        .map(|(pos, tile)| (pos.clone(), part1_step_pos(grid, pos, tile)))
        .collect()
}

fn part1_step_pos(grid: &Grid, pos: &Vector2D, tile: &Tile) -> Tile {
    match tile {
        Tile::Floor => Tile::Floor,
        Tile::Empty => {
            if part1_count_neighbour_seats(grid, pos) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if part1_count_neighbour_seats(grid, pos) >= 4 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
    }
}

fn part1_count_neighbour_seats(grid: &Grid, pos: &Vector2D) -> usize {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            if let Some(Tile::Occupied) = grid.get(&(pos.clone() + Vector2D::new(x, y))) {
                count += 1
            }
        }
    }
    count
}

#[aoc(day11, part1)]
pub fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let new_grid = part1_step(&grid);
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }
    grid.values()
        .filter(|&&tile| tile == Tile::Occupied)
        .count()
}

fn part2_step(grid: &Grid) -> Grid {
    grid.iter()
        .map(|(pos, tile)| (pos.clone(), part2_step_pos(grid, pos, tile)))
        .collect()
}

fn part2_step_pos(grid: &Grid, pos: &Vector2D, tile: &Tile) -> Tile {
    match tile {
        Tile::Floor => Tile::Floor,
        Tile::Empty => {
            if part2_count_neighbour_seats(grid, pos) == 0 {
                Tile::Occupied
            } else {
                Tile::Empty
            }
        }
        Tile::Occupied => {
            if part2_count_neighbour_seats(grid, pos) >= 5 {
                Tile::Empty
            } else {
                Tile::Occupied
            }
        }
    }
}

fn part2_count_neighbour_seats(grid: &Grid, pos: &Vector2D) -> usize {
    let mut count = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            let step = Vector2D::new(x, y);
            let mut next_pos = pos.clone() + step;
            while let Some(Tile::Floor) = grid.get(&next_pos) {
                next_pos += step;
            }
            if let Some(Tile::Occupied) = grid.get(&next_pos) {
                count += 1
            }
        }
    }
    count
}

#[aoc(day11, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    loop {
        let new_grid = part2_step(&grid);
        if new_grid == grid {
            break;
        }
        grid = new_grid;
    }
    grid.values()
        .filter(|&&tile| tile == Tile::Occupied)
        .count()
}
