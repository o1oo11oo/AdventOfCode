use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, u16, u8},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let search = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    input
        .lines()
        .map(parse_sue)
        .find_map(|(id, properties)| {
            properties
                .iter()
                .all(|(prop_name, prop_val)| search.get(prop_name) == Some(prop_val))
                .then_some(id)
        })
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    use Compare::*;
    let search = HashMap::from([
        ("children", Equal(3)),
        ("cats", Greater(7)),
        ("samoyeds", Equal(2)),
        ("pomeranians", Less(3)),
        ("akitas", Equal(0)),
        ("vizslas", Equal(0)),
        ("goldfish", Less(5)),
        ("trees", Greater(3)),
        ("cars", Equal(2)),
        ("perfumes", Equal(1)),
    ]);

    input
        .lines()
        .map(parse_sue)
        .find_map(|(id, properties)| {
            properties
                .iter()
                .all(|(prop_name, prop_val)| match search.get(prop_name) {
                    Some(Less(val)) => prop_val < val,
                    Some(Equal(val)) => prop_val == val,
                    Some(Greater(val)) => prop_val > val,
                    None => false,
                })
                .then_some(id)
        })
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone, Copy)]
enum Compare {
    Less(u8),
    Equal(u8),
    Greater(u8),
}

fn parse_sue(input: &str) -> (u16, HashMap<&str, u8>) {
    type ParseRes<'a> = (u16, Vec<(&'a str, u8)>);
    fn parse(input: &str) -> IResult<&str, ParseRes> {
        tuple((
            delimited(tag("Sue "), u16, tag(": ")),
            separated_list1(tag(", "), tuple((terminated(alpha1, tag(": ")), u8))),
        ))(input)
    }

    let (id, values) = parse(input).finish().unwrap().1;
    let properties = values.into_iter().collect::<HashMap<_, _>>();
    (id, properties)
}
