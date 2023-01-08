use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, u16},
    combinator::map,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|l| {
            let (ins, var) = l.split_once(" -> ").unwrap();
            (var, ins.into())
        })
        .collect::<HashMap<&str, Instruction>>();

    instructions
        .get("a")
        .unwrap()
        .value(&instructions, &mut HashMap::new())
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut instructions = input
        .lines()
        .map(|l| {
            let (ins, var) = l.split_once(" -> ").unwrap();
            (var, ins.into())
        })
        .collect::<HashMap<&str, Instruction>>();

    let a = instructions
        .get("a")
        .unwrap()
        .value(&instructions, &mut HashMap::new());
    instructions.insert("b", Instruction::Identity(Data::Constant(a)));

    instructions
        .get("a")
        .unwrap()
        .value(&instructions, &mut HashMap::new())
        .to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction<'a> {
    Identity(Data<'a>),
    BitAnd(Data<'a>, Data<'a>),
    BitOr(Data<'a>, Data<'a>),
    BitNot(Data<'a>),
    LShift(Data<'a>, Data<'a>),
    RShift(Data<'a>, Data<'a>),
}

impl<'a> Instruction<'a> {
    fn value(
        &self,
        instructions: &'a HashMap<&'a str, Instruction>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        match self {
            Instruction::Identity(d) => d.value(instructions, cache),
            Instruction::BitAnd(a, b) => {
                a.value(instructions, cache) & b.value(instructions, cache)
            }
            Instruction::BitOr(a, b) => a.value(instructions, cache) | b.value(instructions, cache),
            Instruction::BitNot(d) => !d.value(instructions, cache),
            Instruction::LShift(a, b) => {
                a.value(instructions, cache) << b.value(instructions, cache)
            }
            Instruction::RShift(a, b) => {
                a.value(instructions, cache) >> b.value(instructions, cache)
            }
        }
    }
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        fn parse(input: &str) -> IResult<&str, Instruction> {
            alt((
                map(
                    tuple((
                        terminated(map(alphanumeric1, Data::from), tag(" AND ")),
                        map(alphanumeric1, Data::from),
                    )),
                    |(a, b)| Instruction::BitAnd(a, b),
                ),
                map(
                    tuple((
                        terminated(map(alphanumeric1, Data::from), tag(" OR ")),
                        map(alphanumeric1, Data::from),
                    )),
                    |(a, b)| Instruction::BitOr(a, b),
                ),
                map(
                    preceded(tag("NOT "), map(alphanumeric1, Data::from)),
                    Instruction::BitNot,
                ),
                map(
                    tuple((
                        terminated(map(alphanumeric1, Data::from), tag(" LSHIFT ")),
                        map(alphanumeric1, Data::from),
                    )),
                    |(a, b)| Instruction::LShift(a, b),
                ),
                map(
                    tuple((
                        terminated(map(alphanumeric1, Data::from), tag(" RSHIFT ")),
                        map(alphanumeric1, Data::from),
                    )),
                    |(a, b)| Instruction::RShift(a, b),
                ),
                map(map(alphanumeric1, Data::from), Instruction::Identity),
            ))(input)
        }

        parse(value).finish().unwrap().1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Data<'a> {
    Variable(&'a str),
    Constant(u16),
}

impl<'a> Data<'a> {
    fn value(
        &self,
        instructions: &'a HashMap<&'a str, Instruction>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        match self {
            Data::Variable(name) => {
                if let Some(val) = cache.get(name) {
                    *val
                } else {
                    let val = instructions.get(name).unwrap().value(instructions, cache);
                    cache.insert(*name, val);
                    val
                }
            }
            Data::Constant(val) => *val,
        }
    }
}

impl<'a> From<&'a str> for Data<'a> {
    fn from(value: &'a str) -> Self {
        fn parse(input: &str) -> IResult<&str, Data> {
            alt((map(u16, Data::Constant), map(alpha1, Data::Variable)))(input)
        }

        parse(value).finish().unwrap().1
    }
}
