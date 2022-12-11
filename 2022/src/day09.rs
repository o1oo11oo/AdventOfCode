use std::collections::HashSet;

pub(crate) fn part_1(input: &str) -> String {
    let commands = input.lines().flat_map(Command::from);
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut positions = HashSet::new();
    positions.insert(tail);

    for command in commands {
        execute_command(command, &mut head);
        correct_tail(&head, &mut tail);
        positions.insert(tail);
    }

    positions.len().to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let commands = input.lines().flat_map(Command::from);
    let mut knots = vec![Position { x: 0, y: 0 }; 10];
    let mut positions = HashSet::new();
    positions.insert(knots[9]);

    for command in commands {
        execute_command(command, &mut knots[0]);
        for i in 1..knots.len() {
            let (head, tail) = knots.split_at_mut(i);
            correct_tail(head.last().unwrap(), &mut tail[0]);
        }
        positions.insert(knots[9]);
    }

    positions.len().to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

impl Command {
    fn from(value: &str) -> Vec<Self> {
        let mut split = value.split(' ');
        let motion = split.next().unwrap().as_bytes();
        let distance = split.next().unwrap().parse().unwrap();

        match motion[0] {
            b'U' => vec![Self::Up; distance],
            b'D' => vec![Self::Down; distance],
            b'L' => vec![Self::Left; distance],
            b'R' => vec![Self::Right; distance],
            _ => unreachable!(),
        }
    }
}

fn execute_command(command: Command, head: &mut Position) {
    match command {
        Command::Up => head.y += 1,
        Command::Down => head.y -= 1,
        Command::Left => head.x -= 1,
        Command::Right => head.x += 1,
    }
}

fn correct_tail(head: &Position, tail: &mut Position) {
    if head.x - tail.x >= 2 {
        tail.x += 1;

        if head.y - tail.y >= 1 {
            tail.y += 1;
        } else if head.y - tail.y <= -1 {
            tail.y -= 1;
        }
    } else if head.x - tail.x <= -2 {
        tail.x -= 1;

        if head.y - tail.y >= 1 {
            tail.y += 1;
        } else if head.y - tail.y <= -1 {
            tail.y -= 1;
        }
    } else if head.y - tail.y >= 2 {
        tail.y += 1;

        if head.x - tail.x >= 1 {
            tail.x += 1;
        } else if head.x - tail.x <= -1 {
            tail.x -= 1;
        }
    } else if head.y - tail.y <= -2 {
        tail.y -= 1;

        if head.x - tail.x >= 1 {
            tail.x += 1;
        } else if head.x - tail.x <= -1 {
            tail.x -= 1;
        }
    }
}
