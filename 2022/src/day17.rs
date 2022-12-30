use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    let mut movements = input
        .trim()
        .as_bytes()
        .iter()
        .map(WindDirection::from)
        .enumerate()
        .cycle();
    let mut stack = vec![[Tile::Rock; 9]];
    let mut height = 0;

    for count in 0..2022 {
        height = simulate_shape(&mut stack, &mut movements, count, height);
    }

    log::debug!("{}", display_stack(&stack, &[], None, None));

    height.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut movements = input
        .trim()
        .as_bytes()
        .iter()
        .map(WindDirection::from)
        .enumerate()
        .cycle();
    let mut stack = vec![[Tile::Rock; 9]];
    let mut height = 0;
    const AMOUNT: usize = 1000000000000;
    let (cycle_start, cycle_length, cycle_height) = find_cycle(movements.clone());

    for count in 0..cycle_start {
        height = simulate_shape(&mut stack, &mut movements, count, height);
    }

    let remainder = AMOUNT - cycle_start;
    let cycles = remainder / cycle_length;
    let remainder = remainder % cycle_length;

    for count in cycle_start..remainder + cycle_start {
        height = simulate_shape(&mut stack, &mut movements, count, height);
    }

    (cycles * cycle_height + height).to_string()
}

fn assure_free_space(stack: &mut Vec<[Tile; 9]>, height: usize) {
    for _ in 0..8usize.saturating_sub(stack.len() - height) {
        stack.push([
            Tile::Floor,
            Tile::Air,
            Tile::Air,
            Tile::Air,
            Tile::Air,
            Tile::Air,
            Tile::Air,
            Tile::Air,
            Tile::Floor,
        ]);
    }
}

fn simulate_shape(
    stack: &mut Vec<[Tile; 9]>,
    wind: &mut impl Iterator<Item = (usize, WindDirection)>,
    count: usize,
    height: usize,
) -> usize {
    assure_free_space(stack, height);

    let shape: Shape = count.into();
    let mut active = Ok(shape.coordinates(height));

    while active.is_ok() {
        active = simulate_move(stack, active.unwrap(), wind.next().unwrap().1)
    }

    active
        .unwrap_err()
        .iter()
        .map(|c| c.row)
        .max()
        .unwrap()
        .max(height)
}

fn simulate_move(
    stack: &mut [[Tile; 9]],
    active: Vec<Coordinate>,
    wind: WindDirection,
) -> Result<Vec<Coordinate>, Vec<Coordinate>> {
    let mut pos = active;

    let wind_pos = pos.iter().map(|c| c.apply_wind(wind));
    if wind_pos
        .clone()
        .all(|p| stack[p.row][p.column] == Tile::Air)
    {
        pos = wind_pos.collect();
    }

    let down_pos = pos.iter().map(|c| c.apply_wind(WindDirection::Down));
    if down_pos
        .clone()
        .all(|p| stack[p.row][p.column] == Tile::Air)
    {
        Ok(down_pos.collect())
    } else {
        for p in &pos {
            stack[p.row][p.column] = Tile::Rock;
        }
        Err(pos)
    }
}

fn find_cycle(movements: impl Iterator<Item = (usize, WindDirection)>) -> (usize, usize, usize) {
    let mut movements = movements.peekable();
    let mut stack = vec![[Tile::Rock; 9]];
    let mut height = 0;

    let mut cache: HashMap<(u8, usize), (usize, usize)> = HashMap::new();
    let mut start = None;

    for count in 0.. {
        let state = ((count % 5) as u8, movements.peek().unwrap().0);

        if let Some(&(count_old, height_old)) = cache.get(&state) {
            let length = count - count_old;

            if let Some((start_old, start_new)) = start && start_new == count_old {
                log::debug!("Continuous cycle starting at {start_old} detected at ({}, {: >5}) between {count_old: >4} (h: {height_old: >4}) and {count: >4} (h: {height: >4}), length: {length: >4}", state.0, state.1);
                return (start_old, length, height - height_old);
            }

            start = start.or(Some((count_old, count)));
        } else {
            start = None;
        }
        cache.insert(state, (count, height));

        height = simulate_shape(&mut stack, &mut movements, count, height);
    }

    unreachable!()
}

fn display_stack(
    stack: &[[Tile; 9]],
    active: &[Coordinate],
    from: Option<usize>,
    to: Option<usize>,
) -> String {
    let from = from.unwrap_or(0);
    let to = to.unwrap_or(stack.len());

    (from.max(1)..to)
        .rev()
        .flat_map(|row| {
            std::iter::once(format!("\n{row:0>4}")).chain((0..9).map(move |column| {
                if active.contains(&Coordinate { row, column }) {
                    "@".to_string()
                } else {
                    stack[row][column].to_string()
                }
            }))
        })
        .chain(if from == 0 {
            std::iter::once("\n    +-------+".to_string())
        } else {
            std::iter::once("".to_string())
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn apply_wind(&self, wind: WindDirection) -> Coordinate {
        match wind {
            WindDirection::Left => Coordinate {
                row: self.row,
                column: self.column - 1,
            },
            WindDirection::Right => Coordinate {
                row: self.row,
                column: self.column + 1,
            },
            WindDirection::Down => Coordinate {
                row: self.row - 1,
                column: self.column,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Minus,
    Plus,
    L,
    I,
    Square,
}

impl Shape {
    fn coordinates(&self, height: usize) -> Vec<Coordinate> {
        use Shape::*;
        let lower_left = Coordinate {
            row: height + 4,
            column: 3,
        };
        match self {
            Minus => (0..=3)
                .map(|i| Coordinate {
                    row: lower_left.row,
                    column: lower_left.column + i,
                })
                .collect(),
            Plus => [0, 1, 1, 1, 2]
                .iter()
                .zip([1, 0, 1, 2, 1].iter())
                .map(|(i, j)| Coordinate {
                    row: lower_left.row + i,
                    column: lower_left.column + j,
                })
                .collect(),
            L => [0, 0, 0, 1, 2]
                .iter()
                .zip([0, 1, 2, 2, 2].iter())
                .map(|(i, j)| Coordinate {
                    row: lower_left.row + i,
                    column: lower_left.column + j,
                })
                .collect(),
            I => (0..=3)
                .map(|i| Coordinate {
                    row: lower_left.row + i,
                    column: lower_left.column,
                })
                .collect(),
            Square => (0..=1)
                .cartesian_product(0..=1)
                .map(|(i, j)| Coordinate {
                    row: lower_left.row + i,
                    column: lower_left.column + j,
                })
                .collect(),
        }
    }
}

impl From<usize> for Shape {
    fn from(value: usize) -> Self {
        use Shape::*;
        match value % 5 {
            0 => Minus,
            1 => Plus,
            2 => L,
            3 => I,
            4 => Square,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Floor,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => write!(f, "."),
            Tile::Floor => write!(f, "|"),
            Tile::Rock => write!(f, "#"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum WindDirection {
    Left,
    Right,
    Down, // to make down movements easier
}

impl From<&u8> for WindDirection {
    fn from(value: &u8) -> Self {
        use WindDirection::*;
        match value {
            b'<' => Left,
            b'>' => Right,
            _ => unreachable!(),
        }
    }
}
