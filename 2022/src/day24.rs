use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use arrayvec::ArrayVec;
use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    let mut valleys = Valleys::from_input(input);
    let start = valleys.get_start_in(0);
    let target = valleys.get_target();
    log::debug!("valley at start:\n{}", display_grid(valleys.get(0)));

    // instead of A*, BFS works (if you properly manage the queue)
    // instead of manually implementing A*, this generic version could be used:
    // let (path, len) = pathfinding::directed::astar::astar(
    //     &start,
    //     |pos| valleys.reachable(*pos).map(|p| (p, 1)).collect_vec(),
    //     |pos| (pos.0.abs_diff(target.0) + pos.1.abs_diff(target.1)),
    //     |pos| (pos.0, pos.1) == target,
    // )
    // .unwrap();

    let (path, len) = valleys.astar(start, target);
    let moves = path.iter().tuple_windows().map(display_step).join(", ");
    log::debug!("path: {path:?}");
    log::debug!("moves: {moves}");

    len.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut valleys = Valleys::from_input(input);
    let start = valleys.get_start_in(0);
    let target = valleys.get_target();

    let (path1, len1) = valleys.astar(start, target);
    let (path2, len2) = valleys.astar(*path1.last().unwrap(), (start.0, start.1));
    let (_path3, len3) = valleys.astar(*path2.last().unwrap(), target);
    (len1 + len2 + len3).to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valleys<I> {
    cache: Vec<Vec<Vec<Tile>>>,
    iter: I,
}

impl Valleys<itertools::Iterate<Vec<Vec<Tile>>, for<'a> fn(&'a Vec<Vec<Tile>>) -> Vec<Vec<Tile>>>> {
    fn from_input(input: &str) -> Self {
        let valley = input
            .lines()
            .map(|l| l.as_bytes().iter().map(Tile::from).collect_vec())
            .collect_vec();
        Self::from_iter(itertools::iterate(valley, Valleys::next))
    }

    fn next(valley: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        let mut next = valley
            .iter()
            .map(|row| row.iter().map(Tile::empty).collect_vec())
            .collect_vec();
        for (row_idx, row) in valley.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if let Some(winds) = tile.winds() {
                    for wind in winds {
                        let new_pos = wind.next_pos((row_idx, col_idx), valley.len(), row.len());
                        next[new_pos.0][new_pos.1].add_wind(*wind);
                    }
                }
            }
        }

        next
    }
}

impl<I> Valleys<I>
where
    I: Iterator<Item = Vec<Vec<Tile>>>,
{
    fn astar(
        &mut self,
        start: (usize, usize, usize),
        target: (usize, usize),
    ) -> (Vec<(usize, usize, usize)>, usize) {
        // heuristic, simple manhattan distance
        fn h(pos: (usize, usize, usize), target: (usize, usize)) -> usize {
            pos.0.abs_diff(target.0) + pos.1.abs_diff(target.1)
        }

        fn reconstruct_path(
            paths: HashMap<(usize, usize, usize), (usize, usize, usize)>,
            mut target: (usize, usize, usize),
        ) -> Vec<(usize, usize, usize)> {
            let mut total_path = vec![target];
            while let Some(source) = paths.get(&target) {
                target = *source;
                total_path.push(target);
            }
            total_path.reverse();
            total_path
        }

        let mut paths = HashMap::new();
        let mut dist = HashMap::from([(start, 0)]);
        let mut queue = BinaryHeap::from([State {
            cost: h(start, target),
            position: start,
        }]);

        while let Some(State { cost: _, position }) = queue.pop() {
            // if target was found reconstruct path and return it with the amount of steps/minutes
            if (position.0, position.1) == target {
                let path = reconstruct_path(paths, position);
                let len = path.len() - 1; // subtract one for the start
                return (path, len);
            }

            // check all reachable neighbours to see if there's a shorter path than currently known possible
            for neighbor in self.reachable(position) {
                let possible_dist = *dist.get(&position).unwrap_or(&(usize::MAX - 1)) + 1;
                if possible_dist < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                    paths.insert(neighbor, position);
                    dist.insert(neighbor, possible_dist);
                    queue.push(State {
                        cost: possible_dist + h(neighbor, target),
                        position: neighbor,
                    });
                }
            }
        }

        // queue is empty but goal was never reached
        // cannot happen as the graph is infinite
        unreachable!()
    }

    fn from_iter(iter: I) -> Self {
        Self {
            cache: vec![],
            iter,
        }
    }

    fn get_start_in(&mut self, minute: usize) -> (usize, usize, usize) {
        let start_col = self.get(minute)[0]
            .iter()
            .position(|t| *t == Tile::Air)
            .unwrap();
        (0, start_col, minute)
    }

    fn get_target(&mut self) -> (usize, usize) {
        let valley = self.get(0);
        let target_col = valley
            .last()
            .unwrap()
            .iter()
            .position(|t| *t == Tile::Air)
            .unwrap();
        (valley.len() - 1, target_col)
    }

    fn get(&mut self, index: usize) -> &Vec<Vec<Tile>> {
        let missing = (index + 1).saturating_sub(self.cache.len());
        self.cache.reserve(missing);
        for _ in 0..missing {
            self.cache.push(self.iter.next().unwrap());
        }
        &self.cache[index]
    }

    fn reachable(
        &mut self,
        position: (usize, usize, usize),
    ) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        let valley = self.get(position.2 + 1);
        [
            (position.0, position.1, position.2 + 1),
            (position.0.saturating_sub(1), position.1, position.2 + 1),
            (position.0, position.1.saturating_sub(1), position.2 + 1),
            (
                (position.0 + 1).min(valley.len() - 1),
                position.1,
                position.2 + 1,
            ),
            (
                position.0,
                (position.1 + 1).min(valley[position.0].len() - 1),
                position.2 + 1,
            ),
        ]
        .into_iter()
        .sorted()
        .dedup()
        .filter(|p| valley[p.0][p.1] == Tile::Air)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Wall,
    Winds(ArrayVec<Wind, 4>),
}

impl Tile {
    fn empty(&self) -> Self {
        match self {
            Tile::Air | Tile::Winds(_) => Tile::Air,
            Tile::Wall => Tile::Wall,
        }
    }

    fn winds(&self) -> Option<&ArrayVec<Wind, 4>> {
        match self {
            Tile::Air | Tile::Wall => None,
            Tile::Winds(winds) => Some(winds),
        }
    }

    fn add_wind(&mut self, wind: Wind) {
        match self {
            Tile::Air => *self = Tile::Winds([wind].into_iter().collect()),
            Tile::Winds(list) => list.push(wind),
            Tile::Wall => unreachable!(),
        }
    }
}

impl From<&u8> for Tile {
    fn from(value: &u8) -> Self {
        match value {
            b'.' => Tile::Air,
            b'#' => Tile::Wall,
            b'^' => Tile::Winds([Wind::Up].into_iter().collect()),
            b'v' => Tile::Winds([Wind::Down].into_iter().collect()),
            b'<' => Tile::Winds([Wind::Left].into_iter().collect()),
            b'>' => Tile::Winds([Wind::Right].into_iter().collect()),
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Winds(winds) => match winds.len() {
                0 => write!(f, "."),
                1 => winds[0].fmt(f),
                len => write!(f, "{len}"),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Wind {
    Up,
    Down,
    Left,
    Right,
}

impl Wind {
    fn next_pos(&self, pos: (usize, usize), rows: usize, columns: usize) -> (usize, usize) {
        match self {
            Wind::Up => (pos.0.checked_sub(2).unwrap_or(rows - 3) + 1, pos.1),
            Wind::Down => (pos.0 % (rows - 2) + 1, pos.1),
            Wind::Left => (pos.0, pos.1.checked_sub(2).unwrap_or(columns - 3) + 1),
            Wind::Right => (pos.0, pos.1 % (columns - 2) + 1),
        }
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wind::Up => write!(f, "^"),
            Wind::Down => write!(f, "v"),
            Wind::Left => write!(f, "<"),
            Wind::Right => write!(f, ">"),
        }
    }
}

fn display_grid<T: std::fmt::Display>(grid: &[Vec<T>]) -> String {
    grid.iter().map(|row| row.iter().join("")).join("\n")
}

fn display_step(step: (&(usize, usize, usize), &(usize, usize, usize))) -> &'static str {
    match step.0 .0.cmp(&step.1 .0) {
        Ordering::Less => "Down",
        Ordering::Greater => "Up",
        Ordering::Equal => match step.0 .1.cmp(&step.1 .1) {
            Ordering::Less => "Right",
            Ordering::Greater => "Left",
            Ordering::Equal => "Wait",
        },
    }
}

// A* helper for the priority queue
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
