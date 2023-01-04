use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{alpha1, u32},
    combinator::map,
    multi::many1,
    Finish, IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let (map, instructions) = get_map_and_instruction(input);
    let mut pos = Position::from(Shape::Map(&map));

    let mut positions = vec![pos];
    for instruction in instructions {
        log::debug!("\n{}", display_path(&map, &positions));
        let steps = pos.apply_instruction(Shape::Map(&map), &instruction);
        positions.extend(steps);
        pos = *positions.last().unwrap();
    }
    log::debug!("\n{}", display_path(&map, &positions));

    pos.value().to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (map, instructions) = get_map_and_instruction(input);
    let cube = Cube::from(Shape::Map(&map));
    let mut pos = Position::from(Shape::Cube(&cube));

    let mut positions = vec![pos.get_original(&cube)];
    for instruction in instructions {
        log::debug!("\n{}", display_path(&map, &positions));
        let steps = pos.apply_instruction(Shape::Cube(&cube), &instruction);
        positions.extend(steps.iter().map(|s| s.get_original(&cube)));
        pos = positions.last().unwrap().get_on_cube(&cube);
    }
    log::debug!("\n{}", display_path(&map, &positions));

    pos.get_original(&cube).value().to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape<'a> {
    Map(&'a [Vec<Tile>]),
    Cube(&'a Cube),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    sides: Vec<Vec<Vec<Tile>>>,
    connections: HashMap<(usize, Direction), (usize, Direction)>,
    start_positions: Vec<(usize, usize)>,
}

impl<'a> From<Shape<'a>> for Cube {
    fn from(value: Shape) -> Self {
        let map = match value {
            Shape::Map(map) => map,
            Shape::Cube(cube) => return cube.to_owned(),
        };

        fn get_side(map: &[Vec<Tile>], side: usize, start: (usize, usize)) -> Vec<Vec<Tile>> {
            map.iter()
                .skip(start.0)
                .take(side)
                .map(|row| row.iter().copied().skip(start.1).take(side).collect())
                .collect()
        }

        let side_len = if map.len() == 12 { 4 } else { 50 };
        let mut sides = (0..4 * side_len)
            .step_by(side_len)
            .cartesian_product((0..4 * side_len).step_by(side_len))
            .map(|start| (start, get_side(map, side_len, start)))
            .filter(|(_, side)| !side.is_empty() && !side[0].is_empty() && side[0][0] != Tile::Air)
            .enumerate()
            .collect_vec();

        let mut start_points = sides
            .iter()
            .map(|(idx, (start, _))| (*start, *idx))
            .collect::<HashMap<_, _>>();
        let sides = sides.drain(..).map(|(_, (_, side))| side).collect_vec();
        let mut connections = HashMap::new();

        // direct connections
        for (start, start_idx) in start_points.iter() {
            if let (Some(row), column) = (start.0.checked_sub(side_len), start.1)
                && let Some(idx) = start_points.get(&(row, column))
            {
                connections
                .entry((*start_idx, Direction::North))
                .or_insert((*idx, Direction::North));
                connections
                .entry((*idx, Direction::South))
                .or_insert((*start_idx, Direction::South));
            }
            if let (row, Some(column)) = (start.0, start.1.checked_sub(side_len))
                && let Some(idx) = start_points.get(&(row, column))
            {
                connections
                .entry((*start_idx, Direction::West))
                .or_insert((*idx, Direction::West));
                connections
                .entry((*idx, Direction::East))
                .or_insert((*start_idx, Direction::East));
            }
            if let Some(idx) = start_points.get(&(start.0 + side_len, start.1)) {
                connections
                    .entry((*start_idx, Direction::South))
                    .or_insert((*idx, Direction::South));
                connections
                    .entry((*idx, Direction::North))
                    .or_insert((*start_idx, Direction::North));
            }
            if let Some(idx) = start_points.get(&(start.0, start.1 + side_len)) {
                connections
                    .entry((*start_idx, Direction::East))
                    .or_insert((*idx, Direction::East));
                connections
                    .entry((*idx, Direction::West))
                    .or_insert((*start_idx, Direction::West));
            }
        }

        // missing connections
        let mut missing = (0..sides.len())
            .cartesian_product(Direction::all())
            .filter(|k| !connections.contains_key(k))
            .collect::<VecDeque<_>>();
        while let Some((start_idx, start_dir)) = missing.pop_front() {
            if let Some(&(middle_idx, middle_dir)) = connections.get(&(start_idx, start_dir.left()))
                && let Some(&(target_idx, target_dir)) = connections.get(&(middle_idx, middle_dir.right()))
            {
                connections.insert((start_idx, start_dir), (target_idx, target_dir.left()));
            } else if let Some(&(middle_idx, middle_dir)) = connections.get(&(start_idx, start_dir.right()))
                && let Some(&(target_idx, target_dir)) = connections.get(&(middle_idx, middle_dir.left()))
            {
                connections.insert((start_idx, start_dir), (target_idx, target_dir.right()));
            }
            else {
                missing.push_back((start_idx, start_dir));
            }
        }

        Cube {
            sides,
            connections,
            start_positions: start_points
                .drain()
                .sorted_by_key(|p| p.1)
                .map(|p| p.0)
                .collect_vec(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    row: usize,
    column: usize,
    direction: Direction,
    side: usize,
}

impl Position {
    fn apply_instruction(mut self, shape: Shape, instruction: &Instruction) -> Vec<Self> {
        match instruction {
            Instruction::Left => vec![Position {
                row: self.row,
                column: self.column,
                direction: self.direction.left(),
                side: self.side,
            }],
            Instruction::Right => vec![Position {
                row: self.row,
                column: self.column,
                direction: self.direction.right(),
                side: self.side,
            }],
            Instruction::Steps(amount) => {
                let mut intermediates = vec![];
                for _ in 0..*amount {
                    self = self.step(shape);
                    intermediates.push(self);
                }
                intermediates
            }
        }
    }

    fn step(self, shape: Shape) -> Self {
        match shape {
            Shape::Map(map) => self.step_on_map(self, map),
            Shape::Cube(cube) => self.step_on_cube(cube),
        }
    }

    fn step_on_map(self, original: Self, map: &[Vec<Tile>]) -> Self {
        let next = match self.direction {
            Direction::North => Position {
                row: self
                    .row
                    .wrapping_sub(1)
                    .clamp(0, map.len().saturating_sub(1)),
                column: self.column,
                direction: self.direction,
                side: self.side,
            },
            Direction::East => Position {
                row: self.row,
                column: (self.column + 1) % map[0].len(),
                direction: self.direction,
                side: self.side,
            },
            Direction::South => Position {
                row: (self.row + 1) % map.len(),
                column: self.column,
                direction: self.direction,
                side: self.side,
            },
            Direction::West => Position {
                row: self.row,
                column: self
                    .column
                    .wrapping_sub(1)
                    .clamp(0, map[0].len().saturating_sub(1)),
                direction: self.direction,
                side: self.side,
            },
        };

        match map[next.row][next.column] {
            Tile::Air => next.step_on_map(original, map),
            Tile::Open => next,
            Tile::Wall => original,
        }
    }

    fn step_on_cube(self, cube: &Cube) -> Self {
        let side = &cube.sides[self.side];
        let (next_row, next_column) = match self.direction {
            Direction::North => (self.row.checked_sub(1), Some(self.column)),
            Direction::East => (
                Some(self.row),
                (self.column + 1 < side[0].len()).then_some(self.column + 1),
            ),
            Direction::South => (
                (self.row + 1 < side.len()).then_some(self.row + 1),
                Some(self.column),
            ),
            Direction::West => (Some(self.row), self.column.checked_sub(1)),
        };

        match (next_row, next_column) {
            (Some(row), Some(column)) => {
                if side[row][column] == Tile::Open {
                    Position {
                        row,
                        column,
                        direction: self.direction,
                        side: self.side,
                    }
                } else {
                    self
                }
            }
            (Some(row), None) => {
                let (new_side_idx, new_dir) =
                    cube.connections.get(&(self.side, self.direction)).unwrap();
                let side = &cube.sides[*new_side_idx];
                let last = side.len() - 1; // relies on sides being square
                let (row, column) = match (self.direction, new_dir) {
                    (Direction::East, Direction::North) => (last, row),
                    (Direction::East, Direction::East) => (row, 0),
                    (Direction::East, Direction::South) => (0, last - row),
                    (Direction::East, Direction::West) => (last - row, last),
                    (Direction::West, Direction::North) => (last, last - row),
                    (Direction::West, Direction::East) => (last - row, 0),
                    (Direction::West, Direction::South) => (0, row),
                    (Direction::West, Direction::West) => (row, last),
                    _ => unreachable!(),
                };

                if side[row][column] == Tile::Open {
                    Position {
                        row,
                        column,
                        direction: *new_dir,
                        side: *new_side_idx,
                    }
                } else {
                    self
                }
            }
            (None, Some(column)) => {
                let (new_side_idx, new_dir) =
                    cube.connections.get(&(self.side, self.direction)).unwrap();
                let side = &cube.sides[*new_side_idx];
                let last = side.len() - 1; // relies on sides being square
                let (row, column) = match (self.direction, new_dir) {
                    (Direction::North, Direction::North) => (last, column),
                    (Direction::North, Direction::East) => (column, 0),
                    (Direction::North, Direction::South) => (0, last - column),
                    (Direction::North, Direction::West) => (last - column, last),
                    (Direction::South, Direction::North) => (last, last - column),
                    (Direction::South, Direction::East) => (last - column, 0),
                    (Direction::South, Direction::South) => (0, column),
                    (Direction::South, Direction::West) => (column, last),
                    _ => unreachable!(),
                };

                if side[row][column] == Tile::Open {
                    Position {
                        row,
                        column,
                        direction: *new_dir,
                        side: *new_side_idx,
                    }
                } else {
                    self
                }
            }
            (None, None) => unreachable!(),
        }
    }

    fn get_original(self, cube: &Cube) -> Self {
        let start = cube.start_positions[self.side];
        Position {
            row: self.row + start.0,
            column: self.column + start.1,
            direction: self.direction,
            side: 0,
        }
    }

    fn get_on_cube(self, cube: &Cube) -> Self {
        let len = cube.sides[0].len();
        let (side, start) = cube
            .start_positions
            .iter()
            .enumerate()
            .find(|(_, start)| {
                start.0 == (self.row / len) * len && start.1 == (self.column / len) * len
            })
            .unwrap();
        Position {
            row: self.row - start.0,
            column: self.column - start.1,
            direction: self.direction,
            side,
        }
    }

    fn value(&self) -> usize {
        ((self.row + 1) * 1000) + ((self.column + 1) * 4) + self.direction.value()
    }
}

impl<'a> From<Shape<'a>> for Position {
    fn from(value: Shape<'a>) -> Self {
        let column = match value {
            Shape::Map(map) => map[0].iter(),
            Shape::Cube(cube) => cube.sides[0][0].iter(),
        }
        .position(|&t| t == Tile::Open)
        .unwrap();
        Position {
            row: 0,
            column,
            direction: Direction::East,
            side: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> impl Iterator<Item = Direction> + Clone {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .copied()
    }

    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn value(&self) -> usize {
        match self {
            Direction::North => 3,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Open,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => write!(f, " "),
            Tile::Open => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

impl From<&u8> for Tile {
    fn from(value: &u8) -> Self {
        match value {
            b' ' => Tile::Air,
            b'.' => Tile::Open,
            b'#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Steps(usize),
    Left,
    Right,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        Instruction::Steps(value.try_into().unwrap())
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            _ => unreachable!(),
        }
    }
}

fn display_path(map: &[Vec<Tile>], positions: &[Position]) -> String {
    let mut map = map
        .iter()
        .map(|row| row.iter().map(|tile| tile.to_string()).collect_vec())
        .collect_vec();

    for pos in positions {
        map[pos.row][pos.column] = pos.direction.to_string();
    }

    std::iter::Iterator::intersperse(map.iter().map(|row| row.join("")), "\n".to_string()).collect()
}

fn get_map_and_instruction(input: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let instructions = parse_instructions(instructions);
    let mut map = map
        .lines()
        .map(|l| l.as_bytes().iter().map(Tile::from).collect_vec())
        .collect_vec();
    let size = map.iter().map(|l| l.len()).max().unwrap();
    for row in &mut map {
        row.extend(std::iter::repeat(Tile::Air).take(size.saturating_sub(row.len())));
    }

    (map, instructions)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    fn parse_inner(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(alt((
            map(u32, Instruction::from),
            map(alpha1, Instruction::from),
        )))(input)
    }

    parse_inner(input).finish().unwrap().1
}
