use std::str::FromStr;

use crate::util::Vector2D;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

impl Orientation {
    fn step(&self) -> Vector2D {
        match *self {
            Orientation::North => Vector2D::new(0, 1),
            Orientation::South => Vector2D::new(0, -1),
            Orientation::West => Vector2D::new(-1, 0),
            Orientation::East => Vector2D::new(1, 0),
        }
    }
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Orientation::North),
            "S" => Ok(Orientation::South),
            "W" => Ok(Orientation::West),
            "E" => Ok(Orientation::East),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Move(Orientation, i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let value = value.parse::<i32>().expect("invalid value");
        if let Ok(orientation) = action.parse::<Orientation>() {
            Ok(Instruction::Move(orientation, value))
        } else {
            Ok(match action {
                "L" => Instruction::Left(value),
                "R" => Instruction::Right(value),
                "F" => Instruction::Forward(value),
                _ => panic!("invalid action"),
            })
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut pos = Vector2D::new(0, 0);
    let mut orientation = Vector2D::new(1, 0);
    for instruction in input {
        match *instruction {
            Instruction::Move(direction, steps) => {
                pos += direction.step() * steps;
            }
            Instruction::Left(degrees) => {
                orientation = rotate_left(orientation, degrees);
            }
            Instruction::Right(degrees) => {
                orientation = rotate_left(orientation, 360 - degrees);
            }
            Instruction::Forward(steps) => {
                pos += orientation * steps;
            }
        }
    }
    pos.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut ship = Vector2D::new(0, 0);
    let mut waypoint = Vector2D::new(10, 1);
    for instruction in input {
        match *instruction {
            Instruction::Move(direction, steps) => {
                waypoint += direction.step() * steps;
            }
            Instruction::Left(degrees) => {
                waypoint = rotate_left(waypoint, degrees);
            }
            Instruction::Right(degrees) => {
                waypoint = rotate_left(waypoint, 360 - degrees);
            }
            Instruction::Forward(steps) => {
                ship += waypoint * steps;
            }
        }
    }
    ship.manhattan_distance()
}

fn rotate_left(pos: Vector2D, degrees: i32) -> Vector2D {
    match degrees {
        90 => Vector2D::new(-pos.y, pos.x),
        180 => Vector2D::new(-pos.x, -pos.y),
        270 => Vector2D::new(pos.y, -pos.x),
        _ => panic!("invalid degrees: {}", degrees),
    }
}
