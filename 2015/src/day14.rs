use std::{cmp::Ordering, collections::HashMap};

use parse_display::FromStr;

pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|l| l.parse::<Reindeer>().unwrap().distance_after(2503))
        .max()
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let reindeer = input
        .lines()
        .map(|l| l.parse::<Reindeer>().unwrap())
        .collect::<Vec<_>>();
    let mut points = HashMap::<_, u32>::new();

    for time in 1..=2503 {
        for furthest in reindeer
            .iter()
            .map(|r| (r, r.distance_after(time)))
            .fold((vec![], u32::MIN), |mut acc, curr| {
                match curr.1.cmp(&acc.1) {
                    Ordering::Greater => (vec![curr.0], curr.1),
                    Ordering::Equal => {
                        acc.0.push(curr.0);
                        acc
                    }
                    Ordering::Less => acc,
                }
            })
            .0
        {
            *points.entry(furthest).or_default() += 1;
        }
    }

    points.values().max().unwrap().to_string()
}

#[derive(Debug, Clone, FromStr, PartialEq, Eq, Hash)]
#[display("{name} can fly {speed} km/s for {flying_time} seconds, but then must rest for {resting_time} seconds.")]
struct Reindeer {
    name: String,
    speed: u32,
    flying_time: u32,
    resting_time: u32,
}

impl Reindeer {
    fn distance_after(&self, time: u32) -> u32 {
        let cycles = time / self.cycle_length();
        let remaining = time % self.cycle_length();
        let remaining = if remaining < self.flying_time {
            self.speed * remaining
        } else {
            self.cycle_distance()
        };
        cycles * self.cycle_distance() + remaining
    }

    fn cycle_distance(&self) -> u32 {
        self.speed * self.flying_time
    }

    fn cycle_length(&self) -> u32 {
        self.flying_time + self.resting_time
    }
}
