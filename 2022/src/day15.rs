use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::i64 as Num, IResult};

type Num = i64;

pub(crate) fn part_1(input: &str) -> String {
    let coordinates = get_coordinates(input);
    let example = coordinates[0].0.x == 2;
    let target_y = if example { 10 } else { 2000000 };
    let ranges = get_ranges(&coordinates, target_y);
    let beacons: u64 = coordinates
        .iter()
        .filter_map(|(_, b, _)| if b.y == target_y { Some(b) } else { None })
        .sorted()
        .dedup()
        .count()
        .try_into()
        .unwrap();
    let amount = ranges.iter().map(|(s, e)| s.abs_diff(*e) + 1).sum::<u64>();

    (amount - beacons).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let coordinates = get_coordinates(input);
    let example = coordinates[0].0.x == 2;
    let target = if example { 20 } else { 4000000 };

    // technically this would also find points outside the x=0..=target range
    // but there are none and not clamping the values is faster
    let position = (0..=target)
        .find_map(|row| {
            let ranges = get_ranges(&coordinates, row);
            if ranges.len() > 1 {
                Some(Coordinate {
                    x: ranges[0].1 + 1,
                    y: row,
                })
            } else {
                None
            }
        })
        .unwrap();

    (position.x * 4000000 + position.y).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: Num,
    y: Num,
}

impl Coordinate {
    fn dist(&self, other: &Coordinate) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn get_ranges(coordinates: &[(Coordinate, Coordinate, u64)], row: Num) -> Vec<(Num, Num)> {
    coordinates
        .iter()
        .filter_map(|(sensor, _, dist)| {
            let target = Coordinate {
                x: sensor.x,
                y: row,
            };

            let target_dist = sensor.dist(&target);
            let remaining = dist.checked_sub(target_dist);
            remaining.map(|r| (target.x - r as i64, target.x + r as i64))
        })
        .sorted()
        .fold(vec![], |mut acc, el| {
            if let Some(last) = acc.len().checked_sub(1) && acc[last].1 + 1 >= el.0 {
                acc[last] = (el.0.min(acc[last].0), el.1.max(acc[last].1));
            } else {
                acc.push(el)
            }

            acc
        })
}

fn get_coordinates(input: &str) -> Vec<(Coordinate, Coordinate, u64)> {
    input
        .lines()
        .map(|l| {
            let (sensor, beacon) = parse_line(l).unwrap().1;
            (sensor, beacon, sensor.dist(&beacon))
        })
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sx) = Num(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sy) = Num(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, bx) = Num(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, by) = Num(input)?;

    Ok((
        input,
        (Coordinate { x: sx, y: sy }, Coordinate { x: bx, y: by }),
    ))
}
