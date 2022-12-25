use std::collections::VecDeque;

pub(crate) fn part_1(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|l| l.as_bytes().iter().map(Position::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_status(&map, Status::Start).unwrap();
    let end = find_status(&map, Status::End).unwrap();
    bfs(&mut map, start);

    map[end.0][end.1].distance.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|l| l.as_bytes().iter().map(Position::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = find_status(&map, Status::Start).unwrap();
    let end = find_status(&map, Status::End).unwrap();
    map[start.0][start.1] = Position {
        height: 0,
        distance: u16::MAX,
        status: Status::Normal,
    };

    let min = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, pos)| if pos.height == 0 { Some((i, j)) } else { None })
        })
        .map(|pos| {
            let mut map = map.clone();
            map[pos.0][pos.1].distance = 0;
            bfs(&mut map, pos);
            map[end.0][end.1].distance
        })
        .min()
        .unwrap();

    min.to_string()
}

#[derive(Debug, Copy, Clone)]
struct Position {
    height: u8,
    distance: u16,
    status: Status,
}

impl From<&u8> for Position {
    fn from(value: &u8) -> Self {
        match value {
            b'S' => Position {
                height: 0,
                distance: 0,
                status: Status::Start,
            },
            b'E' => Position {
                height: b'z' - b'a',
                distance: u16::MAX,
                status: Status::End,
            },
            _ => Position {
                height: value - b'a',
                distance: u16::MAX,
                status: Status::Normal,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    Start,
    End,
    Normal,
}

fn bfs(map: &mut [Vec<Position>], start: (usize, usize)) {
    let mut stack = VecDeque::new();
    stack.push_back(start);

    while let Some(current_pos) = stack.pop_front() {
        let distance = map[current_pos.0][current_pos.1].distance;

        let reachable = find_reachable(map, current_pos);

        for pos in reachable.iter().filter_map(|p| p.as_ref()) {
            if map[pos.0][pos.1].distance > distance + 1 {
                map[pos.0][pos.1].distance = distance + 1;
                stack.push_back(pos.to_owned());
            }
        }
    }
}

fn find_reachable(map: &[Vec<Position>], position: (usize, usize)) -> [Option<(usize, usize)>; 4] {
    let h = map.len();
    let w = map[0].len();
    let height = map[position.0][position.1].height;
    let higher = |(i, j): (usize, usize)| {
        if height + 1 >= map[i][j].height {
            Some((i, j))
        } else {
            None
        }
    };

    let left_pos = position
        .1
        .checked_sub(1)
        .map(|j| (position.0, j))
        .and_then(higher);

    let up_pos = position
        .0
        .checked_sub(1)
        .map(|i| (i, position.1))
        .and_then(higher);

    let right_pos = if position.1 + 1 >= w {
        None
    } else {
        Some(position.1 + 1)
    }
    .map(|j| (position.0, j))
    .and_then(higher);

    let down_pos = if position.0 + 1 >= h {
        None
    } else {
        Some(position.0 + 1)
    }
    .map(|i| (i, position.1))
    .and_then(higher);

    [left_pos, up_pos, right_pos, down_pos]
}

fn find_status(map: &[Vec<Position>], status: Status) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(i, l)| {
        l.iter()
            .enumerate()
            .find_map(|(j, p)| if p.status == status { Some(j) } else { None })
            .map(|j| (i, j))
    })
}
