use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{eof, map_res},
    multi::{many0, many_till},
    IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let (remaining, elves) = elves(input).unwrap();
    log::debug!("remaining: {remaining}");

    let calorie_totals = elves.iter().map(|e| e.total_calories()).collect::<Vec<_>>();
    let largest = calorie_totals.iter().max().unwrap();

    largest.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (remaining, elves) = elves(input).unwrap();
    log::debug!("remaining: {remaining}");

    let mut calorie_totals = elves.iter().map(|e| e.total_calories()).collect::<Vec<_>>();
    calorie_totals.sort();
    calorie_totals.reverse();
    let total: u32 = calorie_totals[..3].iter().sum();

    total.to_string()
}

#[derive(Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn elves(input: &str) -> IResult<&str, Vec<Elf>> {
    let (input, (elves, _)) = many_till(elf, eof)(input)?;

    Ok((input, elves))
}

fn elf(input: &str) -> IResult<&str, Elf> {
    let (input, items) = many0(food_item)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Elf { calories: items }))
}

fn food_item(input: &str) -> IResult<&str, u32> {
    let (input, count) = map_res(digit1, str::parse)(input)?;
    let (input, _) = alt((tag("\n"), tag("\r\n")))(input)?;

    Ok((input, count))
}
