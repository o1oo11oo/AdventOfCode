use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Num = i16;

pub(crate) fn part_1(input: &str) -> String {
    let elves = get_elves(input);
    let elves = itertools::iterate((elves, Direction::North), |(e, d)| (round(e, *d), d.next()))
        .skip(1)
        .take(10)
        .last()
        .unwrap()
        .0;

    log::debug!("after 10 rounds:\n{}", display_elves(&elves));
    empty_in_bounding_box(&elves).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let elves = get_elves(input);
    let (rounds, (elves, _)) =
        // why write simple for loops when you can use complicated iterator statements?
        itertools::iterate((elves, Direction::North), |(e, d)| (round(e, *d), d.next()))
            .enumerate()
            .tuple_windows()
            .take_while(|((_, (before_elves, _)), (_, (after_elves, _)))| {
                before_elves != after_elves
            })
            .last()
            .unwrap()
            .1;

    log::debug!("after {} rounds:\n{}", rounds + 1, display_elves(&elves));
    (rounds + 1).to_string()
}

fn round(elves: &HashSet<(Num, Num)>, direction: Direction) -> HashSet<(Num, Num)> {
    elves
        .iter()
        .map(|elf| {
            if neighbours(elf).iter().any(|pos| elves.contains(pos)) {
                for direction in direction.all_from() {
                    let neighbours = direction.neighbours(elf);
                    let proposal_target = neighbours[0];
                    if neighbours.iter().all(|pos| !elves.contains(pos)) {
                        return (*elf, proposal_target);
                    }
                }
            }

            (*elf, *elf)
        })
        .fold(HashMap::new(), |mut acc, (source, target)| {
            // since a maximum of two elves can propose one position, an
            // alternative is to collect only one source in the map and add
            // back both sources individually if there is a collision
            acc.entry(target).or_insert(vec![]).push(source);
            acc
        })
        .drain()
        .flat_map(|(target, sources)| {
            if sources.len() == 1 {
                // dirty tricks to make sure the types match on both arms and save some allocations
                std::iter::once(target).chain(vec![])
            } else {
                // this repeats the first element but we're collecting into a HashSet
                std::iter::once(sources[0]).chain(sources)
            }
        })
        .collect()
}

fn neighbours(pos: &(Num, Num)) -> [(Num, Num); 8] {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ]
}

fn empty_in_bounding_box(elves: &HashSet<(Num, Num)>) -> Num {
    let (row_min, row_max, col_min, col_max) = grid_size(elves);
    let row = row_max - row_min + 1;
    let col = col_max - col_min + 1;
    let len: Num = elves.len().try_into().unwrap();
    row * col - len
}

fn grid_size(elves: &HashSet<(Num, Num)>) -> (Num, Num, Num, Num) {
    elves
        .iter()
        .fold((Num::MAX, Num::MIN, Num::MAX, Num::MIN), |acc, &curr| {
            (
                acc.0.min(curr.0),
                acc.1.max(curr.0),
                acc.2.min(curr.1),
                acc.3.max(curr.1),
            )
        })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn all_from(&self) -> impl Iterator<Item = Self> {
        itertools::iterate(*self, Self::next).take(4)
    }

    fn neighbours(&self, pos: &(Num, Num)) -> [(Num, Num); 3] {
        match self {
            Direction::North => [
                (pos.0 - 1, pos.1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 - 1, pos.1 + 1),
            ],
            Direction::South => [
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 - 1),
                (pos.0 + 1, pos.1 + 1),
            ],
            Direction::West => [
                (pos.0, pos.1 - 1),
                (pos.0 - 1, pos.1 - 1),
                (pos.0 + 1, pos.1 - 1),
            ],
            Direction::East => [
                (pos.0, pos.1 + 1),
                (pos.0 - 1, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ],
        }
    }
}

fn get_elves(input: &str) -> HashSet<(Num, Num)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(column, ch)| {
                    (*ch == b'#').then_some((row as Num, column as Num))
                })
        })
        .collect()
}

fn display_elves(elves: &HashSet<(Num, Num)>) -> String {
    let (row_min, row_max, col_min, col_max) = grid_size(elves);

    let mut grid = vec![
        vec!["."; (col_max - col_min + 1).try_into().unwrap()];
        (row_max - row_min + 1).try_into().unwrap()
    ];

    for elf in elves {
        let row: usize = (elf.0 - row_min).try_into().unwrap();
        let col: usize = (elf.1 - col_min).try_into().unwrap();
        grid[row][col] = "#";
    }

    grid.iter().map(|row| row.join("")).join("\n")
}
