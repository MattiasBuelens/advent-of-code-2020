use std::collections::HashSet;

use crate::util::Vector3D;

#[derive(Debug, Clone)]
pub struct PocketDimension {
    cubes: HashSet<Vector3D>,
}

impl PocketDimension {
    fn step(&self) -> Self {
        let min_x = self.cubes.iter().map(|pos| pos.x).min().unwrap();
        let max_x = self.cubes.iter().map(|pos| pos.x).max().unwrap();
        let min_y = self.cubes.iter().map(|pos| pos.y).min().unwrap();
        let max_y = self.cubes.iter().map(|pos| pos.y).max().unwrap();
        let min_z = self.cubes.iter().map(|pos| pos.z).min().unwrap();
        let max_z = self.cubes.iter().map(|pos| pos.z).max().unwrap();

        let cubes = ((min_x - 1)..=(max_x + 1))
            .flat_map(move |x| {
                ((min_y - 1)..=(max_y + 1)).flat_map(move |y| {
                    ((min_z - 1)..=(max_z + 1)).filter_map(move |z| {
                        let pos = Vector3D::new(x, y, z);
                        let was_active = self.cubes.contains(&pos);
                        let neighbour_count = get_neighbours(&pos)
                            .into_iter()
                            .filter(|neighbour| self.cubes.contains(neighbour))
                            .count();
                        let is_active = match (was_active, neighbour_count) {
                            // If a cube is active and exactly 2 or 3 of its neighbors are also
                            // active, the cube remains active.
                            (true, 2) => true,
                            (true, 3) => true,
                            // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
                            (false, 3) => true,
                            // Otherwise, the cube remains inactive.
                            _ => false,
                        };
                        if is_active {
                            Some(pos)
                        } else {
                            None
                        }
                    })
                })
            })
            .collect::<HashSet<_>>();
        Self { cubes }
    }
}

fn get_neighbours(pos: &Vector3D) -> Vec<Vector3D> {
    let neighbours = ((pos.x - 1)..=(pos.x + 1)).flat_map(move |x| {
        ((pos.y - 1)..=(pos.y + 1)).flat_map(move |y| {
            ((pos.z - 1)..=(pos.z + 1)).filter_map(move |z| {
                let neighbour_pos = Vector3D::new(x, y, z);
                if &neighbour_pos == pos {
                    None
                } else {
                    Some(neighbour_pos)
                }
            })
        })
    });
    neighbours.collect()
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> PocketDimension {
    let cubes = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => Some(Vector3D::new(x as i32, y as i32, 0)),
                    '.' => None,
                    c => panic!("invalid cube state: {}", c),
                })
        })
        .collect::<HashSet<_>>();
    PocketDimension { cubes }
}

#[aoc(day17, part1)]
pub fn part1(input: &PocketDimension) -> usize {
    let mut dimension = input.clone();
    for i in 0..6 {
        dimension = dimension.step();
    }
    dimension.cubes.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &PocketDimension) -> i32 {
    todo!()
}
