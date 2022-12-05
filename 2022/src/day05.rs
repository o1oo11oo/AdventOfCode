use std::str::Lines;

use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult};

pub(crate) fn part_1(input: &str) -> String {
    let mut lines = input.lines();
    let mut state = get_state(&mut lines);
    let instructions = get_instructions(lines);

    for ins in instructions {
        log::debug!("state: {state:#?}");
        apply_instruction_9k(&mut state, ins);
    }
    log::debug!("state: {state:#?}");

    state
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut lines = input.lines();
    let mut state = get_state(&mut lines);
    let instructions = get_instructions(lines);

    for ins in instructions {
        log::debug!("state: {state:#?}");
        apply_instruction_9k1(&mut state, ins);
    }
    log::debug!("state: {state:#?}");

    state
        .iter_mut()
        .filter_map(|stack| stack.pop())
        .collect::<String>()
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn get_state(lines: &mut Lines) -> Vec<Vec<char>> {
    let mut state = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(state_line_to_stack_contents)
        .collect::<Vec<_>>();

    log::debug!("state: {state:#?}");
    drop(state.pop());
    state.reverse();

    (0..state[0].len())
        .map(|i| {
            state
                .iter()
                .filter_map(|inner| {
                    if inner[i] != ' ' {
                        Some(inner[i])
                    } else {
                        None
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect()
}

fn state_line_to_stack_contents(line: &str) -> Vec<char> {
    line.as_bytes()
        .chunks(4)
        .map(|chunk| chunk[1] as char)
        .collect()
}

fn get_instructions(lines: Lines) -> Vec<Instruction> {
    lines.map(instruction_line_to_ins).collect()
}

fn instruction_line_to_ins(line: &str) -> Instruction {
    let (_, (amount, from, to)) = parse_instruction_line(line).unwrap();
    // subtract 1 from indices but not amount
    Instruction {
        amount,
        from: from - 1,
        to: to - 1,
    }
}

fn parse_instruction_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = map_res(digit1, str::parse)(input)?;

    Ok((input, (amount, from, to)))
}

fn apply_instruction_9k(state: &mut [Vec<char>], ins: Instruction) {
    if ins.from < ins.to {
        let (first, second) = state.split_at_mut(ins.to);
        let source = &mut first[ins.from];
        let target = &mut second[0];
        target.extend(source.drain(source.len() - ins.amount..).rev());
    } else {
        let (first, second) = state.split_at_mut(ins.from);
        let source = &mut second[0];
        let target = &mut first[ins.to];
        target.extend(source.drain(source.len() - ins.amount..).rev());
    }
}

fn apply_instruction_9k1(state: &mut [Vec<char>], ins: Instruction) {
    if ins.from < ins.to {
        let (first, second) = state.split_at_mut(ins.to);
        let source = &mut first[ins.from];
        let target = &mut second[0];
        target.extend(source.drain(source.len() - ins.amount..));
    } else {
        let (first, second) = state.split_at_mut(ins.from);
        let source = &mut second[0];
        let target = &mut first[ins.to];
        target.extend(source.drain(source.len() - ins.amount..));
    }
}
