use std::collections::HashSet;

use crate::util::Vector2D;

#[derive(Debug)]
pub struct Map {
    width: i32,
    height: i32,
    trees: HashSet<Vector2D>,
}

impl Map {
    pub fn trees_on_slope(&self, slope: Vector2D) -> usize {
        let mut pos = Vector2D::zero();
        let mut trees = 0;
        while pos.y < self.height {
            if self.trees.contains(&pos) {
                trees += 1;
            }
            pos += slope;
            pos.x %= self.width;
        }
        trees
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let width = input.lines().next().unwrap().chars().count() as i32;
    let height = input.lines().count() as i32;
    let trees = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Vector2D::new(x as i32, y as i32))
        })
        .collect::<HashSet<_>>();
    Map {
        width,
        height,
        trees,
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &Map) -> usize {
    input.trees_on_slope(Vector2D::new(3, 1))
}

#[aoc(day3, part2)]
pub fn part2(input: &Map) -> usize {
    vec![
        Vector2D::new(1, 1),
        Vector2D::new(3, 1),
        Vector2D::new(5, 1),
        Vector2D::new(7, 1),
        Vector2D::new(1, 2),
    ]
    .into_iter()
    .map(|slope| input.trees_on_slope(slope))
    .product()
}
