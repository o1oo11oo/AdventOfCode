use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|i| i.split_once('\n').unwrap())
        .map(|(left, right)| {
            (
                parse_packet(left).unwrap().1,
                parse_packet(right).unwrap().1,
            )
        })
        .map(|(left, right)| left.cmp(&right))
        .enumerate()
        .filter_map(|(i, o)| {
            if o == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| parse_packet(line).unwrap().1)
        .collect::<Vec<_>>();

    let start = Packet::List(vec![Packet::List(vec![Packet::Item(2)])]);
    let end = Packet::List(vec![Packet::List(vec![Packet::Item(6)])]);

    packets.push(start.clone());
    packets.push(end.clone());
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|p| {
            if p.1 == &start || p.1 == &end {
                Some(p.0 + 1)
            } else {
                None
            }
        })
        .product::<usize>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Item(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Item(left_num) => match other {
                Packet::Item(right_num) => left_num.cmp(right_num),
                Packet::List(_) => Packet::List(vec![self.to_owned()]).cmp(other),
            },
            Packet::List(left_list) => match other {
                Packet::Item(_) => self.cmp(&Packet::List(vec![other.to_owned()])),
                Packet::List(right_list) => {
                    for i in 0..left_list.len().min(right_list.len()) {
                        let ord = left_list[i].cmp(&right_list[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    left_list.len().cmp(&right_list.len())
                }
            },
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(digit1, |i: &str| Packet::Item(i.parse().unwrap())),
        map(
            delimited(
                tag("["),
                many0(terminated(parse_packet, opt(tag(",")))),
                tag("]"),
            ),
            Packet::List,
        ),
    ))(input)
}
