use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{eof, map_res},
    multi::{many0, many_till},
    IResult,
};

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let example =
        std::fs::read_to_string("example.txt").expect("Should have been able to read the file");
    let input =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let (remaining, elves) = elves(input).unwrap();
    log::debug!("remaining: {remaining}");

    let calorie_totals = elves.iter().map(|e| e.total_calories()).collect::<Vec<_>>();
    let largest = calorie_totals.iter().max().unwrap();
    log::info!("Largest: {largest}");
}

fn part_2(input: &str) {
    let (remaining, elves) = elves(input).unwrap();
    log::debug!("remaining: {remaining}");

    let mut calorie_totals = elves.iter().map(|e| e.total_calories()).collect::<Vec<_>>();
    calorie_totals.sort();
    calorie_totals.reverse();
    let total: u32 = calorie_totals[..3].iter().sum();
    log::info!("Total: {total}");
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
