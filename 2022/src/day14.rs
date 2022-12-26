use std::fmt::Display;

pub(crate) fn part_1(input: &str) -> String {
    let (mut area, spawn) = parse_input(input);
    let mut count = 0u32;

    while simulate_one_sand(&mut area, spawn).is_some() {
        count += 1;
    }

    log::debug!("Resulting area:\n{}", format_area(&area));
    count.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (mut area, spawn) = parse_input(input);

    let left = area.len().saturating_sub(spawn.0) - 1;
    let right = area.len().saturating_sub(area[0].len() - spawn.0);
    let spawn = (spawn.0 + left, spawn.1);
    widen_area(&mut area, left, right, true);

    let mut count = 1u32;
    while simulate_one_sand(&mut area, spawn) != Some(spawn) {
        count += 1;
    }

    log::debug!("Resulting area:\n{}", format_area(&area));
    count.to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Material {
    Air,
    Stone,
    Sand,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Air => write!(f, "."),
            Material::Stone => write!(f, "#"),
            Material::Sand => write!(f, "o"),
        }
    }
}

fn simulate_one_sand(area: &mut [Vec<Material>], spawn: (usize, usize)) -> Option<(usize, usize)> {
    let (mut x, mut y) = spawn;
    loop {
        if y + 1 >= area.len() {
            return None;
        }

        if area[y + 1][x] == Material::Air {
            y += 1;
        } else if area[y + 1][x - 1] == Material::Air {
            y += 1;
            x -= 1;
        } else if area[y + 1][x + 1] == Material::Air {
            y += 1;
            x += 1;
        } else {
            area[y][x] = Material::Sand;
            return Some((x, y));
        }
    }
}

fn widen_area(area: &mut [Vec<Material>], left: usize, right: usize, floor: bool) {
    for line in &mut *area {
        *line = std::iter::repeat(Material::Air)
            .take(left)
            .chain(line.drain(..))
            .chain(std::iter::repeat(Material::Air).take(right))
            .collect();
    }

    if floor {
        for x in area.last_mut().unwrap() {
            *x = Material::Stone;
        }
    }
}

fn format_area(area: &[Vec<Material>]) -> String {
    area.iter()
        .map(|line| line.iter().map(|m| m.to_string()).collect::<String>())
        .fold(String::new(), |acc, s| acc + &s + "\n")
}

fn parse_input(input: &str) -> (Vec<Vec<Material>>, (usize, usize)) {
    let ((x_min, x_max), (_y_min, y_max)) = input
        .lines()
        .flat_map(|l| l.split(" -> "))
        .map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .fold(
            ((usize::MAX, usize::MIN), (usize::MAX, usize::MIN)),
            |((x_min, x_max), (y_min, y_max)), (x, y)| {
                ((x_min.min(x), x_max.max(x)), (y_min.min(y), y_max.max(y)))
            },
        );

    let shift = x_min.saturating_sub(2);
    let x_max = x_max - shift + 1;
    let spawn = (500 - shift, 0);

    let pairs = input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let (x, y) = s.split_once(',').unwrap();
                    (
                        x.parse::<usize>().unwrap() - shift,
                        y.parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
                .windows(2)
                .map(|arr| {
                    let x_min = arr[0].0.min(arr[1].0);
                    let x_max = arr[0].0.max(arr[1].0);
                    let y_min = arr[0].1.min(arr[1].1);
                    let y_max = arr[0].1.max(arr[1].1);
                    (x_min..=x_max, y_min..=y_max)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut area = vec![vec![Material::Air; x_max + 2]; y_max + 3];

    for (x_range, y_range) in pairs {
        for x in x_range {
            for y in y_range.clone() {
                area[y][x] = Material::Stone
            }
        }
    }

    (area, spawn)
}
