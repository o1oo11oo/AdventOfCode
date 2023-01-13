use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    sequence::{terminated, tuple},
    Finish, IResult,
};
use rayon::prelude::*;

pub(crate) fn part_1(input: &str) -> String {
    find_max(input.lines().map(parse_line).collect()).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut happiness = input.lines().map(parse_line).collect::<HashMap<_, _>>();
    for name in happiness.keys().map(|k| k.0).sorted().dedup() {
        happiness.insert(("me", name), 0);
        happiness.insert((name, "me"), 0);
    }

    find_max(happiness).to_string()
}

fn find_max(happiness: HashMap<(&str, &str), i64>) -> i64 {
    let names = happiness.keys().map(|k| k.0).sorted().dedup().collect_vec();
    names
        .iter()
        .copied()
        .permutations(names.len())
        .map(|mut permutation| {
            permutation.extend_from_within(..1);
            permutation
        })
        .collect_vec()
        .par_iter()
        .map(|seating| {
            seating
                .iter()
                .copied()
                .tuple_windows()
                .map(|(a, b)| happiness.get(&(a, b)).unwrap() + happiness.get(&(b, a)).unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

fn parse_line(input: &str) -> ((&str, &str), i64) {
    fn parse(input: &str) -> IResult<&str, (&str, &str, i64, &str)> {
        tuple((
            terminated(alpha1, tag(" would ")),
            terminated(alt((tag("gain"), tag("lose"))), tag(" ")),
            terminated(i64, tag(" happiness units by sitting next to ")),
            terminated(alpha1, tag(".")),
        ))(input)
    }

    let res = parse(input).finish().unwrap().1;
    if res.1 == "lose" {
        ((res.0, res.3), -res.2)
    } else {
        ((res.0, res.3), res.2)
    }
}
