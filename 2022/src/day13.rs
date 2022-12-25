use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u8,
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

    let s = packets.binary_search(&start).unwrap();
    let e = packets.binary_search(&end).unwrap();

    ((s + 1) * (e + 1)).to_string()
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
        use Packet::*;
        match (self, other) {
            (Item(left_num), Item(right_num)) => left_num.cmp(right_num),
            (Item(_), List(_)) => List(vec![self.to_owned()]).cmp(other),
            (List(_), Item(_)) => self.cmp(&List(vec![other.to_owned()])),
            // left_list.cmp(right_list) would be sufficient
            (List(left_list), List(right_list)) => left_list
                .iter()
                .zip(right_list.iter())
                .map(|(l, r)| l.cmp(r))
                .find(|&o| o != Ordering::Equal)
                .unwrap_or_else(|| left_list.len().cmp(&right_list.len())),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(u8, Packet::Item),
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
