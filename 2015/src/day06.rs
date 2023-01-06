use std::ops::RangeInclusive;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u16,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let mut lights = vec![vec![false; 1000]; 1000];
    for action in input.lines().map(Action::from) {
        match action {
            Action::TurnOn(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] = true;
                }),
            Action::Toggle(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] = !lights[row][col];
                }),
            Action::TurnOff(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] = false;
                }),
        };
    }

    lights
        .iter()
        .flat_map(|row| row.iter())
        .filter(|l| **l)
        .count()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut lights = vec![vec![0_u32; 1000]; 1000];
    for action in input.lines().map(Action::from) {
        match action {
            Action::TurnOn(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] += 1;
                }),
            Action::Toggle(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] += 2;
                }),
            Action::TurnOff(row_range, col_range) => row_range
                .cartesian_product(col_range)
                .for_each(|(row, col)| {
                    lights[row][col] = lights[row][col].saturating_sub(1);
                }),
        };
    }

    lights
        .iter()
        .flat_map(|row| row.iter())
        .sum::<u32>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    TurnOn(RangeInclusive<usize>, RangeInclusive<usize>),
    Toggle(RangeInclusive<usize>, RangeInclusive<usize>),
    TurnOff(RangeInclusive<usize>, RangeInclusive<usize>),
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        fn parse(input: &str) -> IResult<&str, (&str, u16, u16, u16, u16)> {
            tuple((
                alt((tag("turn on "), tag("toggle "), tag("turn off "))),
                u16,
                delimited(tag(","), u16, tag(" through ")),
                terminated(u16, tag(",")),
                u16,
            ))(input)
        }

        let res = parse(value).finish().unwrap().1;
        match res.0 {
            "turn on " => Action::TurnOn(res.1.into()..=res.3.into(), res.2.into()..=res.4.into()),
            "toggle " => Action::Toggle(res.1.into()..=res.3.into(), res.2.into()..=res.4.into()),
            "turn off " => {
                Action::TurnOff(res.1.into()..=res.3.into(), res.2.into()..=res.4.into())
            }
            _ => unreachable!(),
        }
    }
}
