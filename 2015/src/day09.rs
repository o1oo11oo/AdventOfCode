use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    sequence::{delimited, tuple},
    Finish, IResult,
};
use rayon::prelude::*;

pub(crate) fn part_1(input: &str) -> String {
    distances(parse_input(input)).0.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    distances(parse_input(input)).1.to_string()
}

fn distances((locations, distances): (Vec<&str>, HashMap<(&str, &str), u32>)) -> (u32, u32) {
    locations
        .iter()
        .copied()
        .permutations(locations.len())
        .collect_vec()
        .par_iter()
        .map(|route| {
            route
                .iter()
                .copied()
                .tuple_windows()
                .map(|c| distances.get(&c).unwrap())
                .sum::<u32>()
        })
        .map(|i| (i, i))
        .reduce(
            || (u32::MAX, u32::MIN),
            |acc, curr| (acc.0.min(curr.0), acc.1.max(curr.1)),
        )
}

fn parse_input(input: &str) -> (Vec<&str>, HashMap<(&str, &str), u32>) {
    let distances = input
        .lines()
        .flat_map(parse_connections)
        .collect::<HashMap<_, _>>();
    (
        distances.keys().map(|k| k.0).sorted().dedup().collect_vec(),
        distances,
    )
}

fn parse_connections(input: &str) -> impl Iterator<Item = ((&str, &str), u32)> {
    fn parse(input: &str) -> IResult<&str, ((&str, &str), u32)> {
        tuple((
            tuple((alpha1, delimited(tag(" to "), alpha1, tag(" = ")))),
            u32,
        ))(input)
    }

    let res = parse(input).finish().unwrap().1;
    std::iter::once(res).chain(std::iter::once(((res.0 .1, res.0 .0), res.1)))
}
