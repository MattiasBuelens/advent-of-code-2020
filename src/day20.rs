use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

use crate::util::Vector2D;

const TILE_SIZE: usize = 10;

const SEA_MONSTER: &'static str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    grid: [[bool; TILE_SIZE]; TILE_SIZE],
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
    fn border_top(&self) -> [bool; TILE_SIZE] {
        self.grid[0]
    }

    fn border_bottom(&self) -> [bool; TILE_SIZE] {
        self.grid[TILE_SIZE - 1]
    }

    fn border_left(&self) -> [bool; TILE_SIZE] {
        let mut border = [false; TILE_SIZE];
        for y in 0..TILE_SIZE {
            border[y] = self.grid[y][0];
        }
        border
    }

    fn border_right(&self) -> [bool; TILE_SIZE] {
        let mut border = [false; TILE_SIZE];
        for y in 0..TILE_SIZE {
            border[y] = self.grid[y][TILE_SIZE - 1];
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
        for y in 0..TILE_SIZE {
            tile.grid[y].reverse()
        }
        tile
    }

    fn rotate_left(&self) -> Self {
        let mut tile = Self::default();
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                tile.grid[y][x] = self.grid[x][TILE_SIZE - 1 - y];
            }
        }
        tile
    }

    fn rotate_right(&self) -> Self {
        let mut tile = Self::default();
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                tile.grid[y][x] = self.grid[TILE_SIZE - 1 - x][y];
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

fn place_tiles(size: i32, input: &Input) -> Option<HashMap<Vector2D, (u32, Tile)>> {
    solve(
        &HashMap::new(),
        size,
        Vector2D::zero(),
        HashMap::from_iter(input.clone()),
    )
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> u64 {
    let size = (input.len() as f32).sqrt() as i32;
    let solution = place_tiles(size, input).unwrap();

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

type Image = Vec<Vec<bool>>;

fn create_image(size: usize, tiles: &HashMap<Vector2D, (u32, Tile)>) -> Image {
    let mut image = vec![vec![false; size * (TILE_SIZE - 2)]; size * (TILE_SIZE - 2)];
    for y in 0..size {
        for x in 0..size {
            let (_, tile) = tiles.get(&Vector2D::new(x as i32, y as i32)).unwrap();
            for (tile_y, tile_row) in tile.grid.iter().skip(1).take(TILE_SIZE - 2).enumerate() {
                let image_y = y * (TILE_SIZE - 2) + tile_y;
                let image_row = image.get_mut(image_y).unwrap();
                for (tile_x, cell) in tile_row.iter().skip(1).take(TILE_SIZE - 2).enumerate() {
                    let image_x = x * (TILE_SIZE - 2) + tile_x;
                    image_row[image_x] = *cell;
                }
            }
        }
    }
    image
}

fn create_sea_monster_pattern() -> Vec<Vector2D> {
    SEA_MONSTER
        .lines()
        .skip(1)
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '#' {
                    Some(Vector2D::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn flip_vertical(image: &Image) -> Image {
    let mut image = image.clone();
    image.reverse();
    image
}

fn flip_horizontal(image: &Image) -> Image {
    let mut image = image.clone();
    for row in image.iter_mut() {
        row.reverse()
    }
    image
}

fn rotate_left(image: &Image) -> Image {
    let size = image.len();
    let mut new_image = image.clone();
    for y in 0..size {
        for x in 0..size {
            new_image[y][x] = image[x][size - 1 - y];
        }
    }
    new_image
}

fn rotate_right(image: &Image) -> Image {
    let size = image.len();
    let mut new_image = image.clone();
    for y in 0..size {
        for x in 0..size {
            new_image[y][x] = image[size - 1 - x][y];
        }
    }
    new_image
}

fn image_permutations(image: &Image) -> Vec<Image> {
    vec![
        image.clone(),
        rotate_right(image),
        rotate_right(&rotate_right(image)),
        rotate_left(image),
        flip_vertical(image),
        rotate_right(&flip_vertical(image)),
        rotate_right(&rotate_right(&flip_vertical(image))),
        rotate_left(&flip_vertical(image)),
    ]
}

fn count_image_pattern(image: &Image, pattern: &Vec<Vector2D>) -> usize {
    let pattern_width = pattern.iter().max_by_key(|pos| pos.x).unwrap().x as usize;
    let pattern_height = pattern.iter().max_by_key(|pos| pos.y).unwrap().y as usize;
    let mut count = 0;
    for start_y in 0..(image.len() - pattern_height) {
        for start_x in 0..(image[0].len() - pattern_width) {
            let is_match = pattern
                .iter()
                .all(|pos| image[start_y + pos.y as usize][start_x + pos.x as usize]);
            if is_match {
                count += 1;
            }
        }
    }
    count
}

#[allow(unused)]
fn print_image(image: &Image) {
    for row in image {
        for &cell in row {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> usize {
    let size = (input.len() as f32).sqrt() as i32;
    let tiles = place_tiles(size, input).unwrap();

    let image = create_image(size as usize, &tiles);
    let pattern = create_sea_monster_pattern();

    let count_things = image
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell).count())
        .sum::<usize>();

    for perm in image_permutations(&image) {
        let monsters = count_image_pattern(&perm, &pattern);
        if monsters > 0 {
            return count_things - (monsters * pattern.len());
        }
    }
    panic!("no monsters found")
}
