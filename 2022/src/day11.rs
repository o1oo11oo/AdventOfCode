use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

type WorryLevel = u64;

pub(crate) fn part_1(input: &str) -> String {
    let mut monkeys = parse_input(input).finish().unwrap().1;

    let res = simulate_rounds(20, &mut monkeys, None);

    res.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut monkeys = parse_input(input).finish().unwrap().1;
    let remainder_class = monkeys.iter().map(|m| m.test).product();

    let res = simulate_rounds(10000, &mut monkeys, Some(remainder_class));

    res.to_string()
}

fn simulate_rounds(
    amount: usize,
    monkeys: &mut [Monkey],
    remainder_class: Option<WorryLevel>,
) -> usize {
    for _ in 1..=amount {
        simulate_round(monkeys, remainder_class);
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn simulate_round(monkeys: &mut [Monkey], remainder_class: Option<WorryLevel>) {
    for id in 0..monkeys.len() {
        monkeys[id].inspections += monkeys[id].items.len();
        let targets = get_targets(&mut monkeys[id], remainder_class);
        apply_targets(monkeys, targets);
    }
}

fn get_targets(
    monkey: &mut Monkey,
    remainder_class: Option<WorryLevel>,
) -> Vec<(WorryLevel, usize)> {
    monkey
        .items
        .drain(..)
        .map(|item| {
            let mut worry_level = monkey.operation.apply(item);

            if let Some(remainder_class) = remainder_class {
                worry_level %= remainder_class;
            } else {
                worry_level /= 3;
            }

            let target_id = if worry_level % monkey.test == 0 {
                monkey.success_id
            } else {
                monkey.fail_id
            };

            (worry_level, target_id)
        })
        .collect()
}

fn apply_targets(monkeys: &mut [Monkey], targets: Vec<(WorryLevel, usize)>) {
    for (worry_level, target_id) in targets {
        monkeys[target_id].items.push(worry_level);
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Operation,
    test: WorryLevel,
    success_id: usize,
    fail_id: usize,
    inspections: usize,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Squared,
}

impl Operation {
    fn apply(&self, other: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(amount) => amount + other,
            Operation::Multiply(amount) => amount * other,
            Operation::Squared => other * other,
        }
    }
}

impl From<(&str, &str, &str)> for Operation {
    fn from(value: (&str, &str, &str)) -> Self {
        let (operand, _, amount) = value;
        if let Ok(amount) = amount.parse() {
            match operand {
                "+" => Operation::Add(amount),
                "*" => Operation::Multiply(amount),
                _ => unreachable!(),
            }
        } else {
            Operation::Squared
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    many1(terminated(parse_monkey, many0(line_ending)))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), digit1, tag(":\n"))(input)?;
    let (input, items) = delimited(
        tag("  Starting items: "),
        many1(map_res(terminated(digit1, opt(tag(", "))), str::parse)),
        line_ending,
    )(input)?;
    let (input, operation) = map(
        delimited(
            tag("  Operation: new = old "),
            tuple((take(1usize), tag(" "), take_until("\n"))),
            line_ending,
        ),
        Operation::from,
    )(input)?;
    let (input, test) = delimited(
        tag("  Test: divisible by "),
        map_res(digit1, str::parse),
        line_ending,
    )(input)?;
    let (input, success_id) = delimited(
        tag("    If true: throw to monkey "),
        map_res(digit1, str::parse),
        line_ending,
    )(input)?;
    let (input, fail_id) = delimited(
        tag("    If false: throw to monkey "),
        map_res(digit1, str::parse),
        line_ending,
    )(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            success_id,
            fail_id,
            inspections: 0,
        },
    ))
}
