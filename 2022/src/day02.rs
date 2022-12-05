use nom::{
    bytes::complete::take,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let (_, games) = games_1(input).unwrap();
    log::debug!("{games:#?}");
    let scores = games.iter().map(|g| g.score()).collect::<Vec<_>>();
    log::debug!("scores: {scores:?}");
    let total: u32 = scores.iter().sum();

    total.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (_, games) = games_2(input).unwrap();
    log::debug!("{games:#?}");
    let scores = games.iter().map(|g| g.score()).collect::<Vec<_>>();
    log::debug!("scores: {scores:?}");
    let total: u32 = scores.iter().sum();

    total.to_string()
}

#[derive(Debug)]
struct Game {
    opponent_move: Move,
    own_move: Move,
}

#[allow(clippy::identity_op)]
impl Game {
    fn score(&self) -> u32 {
        match (&self.own_move, &self.opponent_move) {
            (Move::Rock, Move::Rock) => 1 + 3,
            (Move::Rock, Move::Paper) => 1 + 0,
            (Move::Rock, Move::Scissors) => 1 + 6,
            (Move::Paper, Move::Rock) => 2 + 6,
            (Move::Paper, Move::Paper) => 2 + 3,
            (Move::Paper, Move::Scissors) => 2 + 0,
            (Move::Scissors, Move::Rock) => 3 + 0,
            (Move::Scissors, Move::Paper) => 3 + 6,
            (Move::Scissors, Move::Scissors) => 3 + 3,
        }
    }
}

impl From<(Move, Move)> for Game {
    fn from((move1, move2): (Move, Move)) -> Self {
        Game {
            opponent_move: move1,
            own_move: move2,
        }
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_move(value: &str) -> Self {
        match value {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_answer(value: &str, other: &Move) -> Self {
        match (value, other) {
            ("X", Move::Rock) => Move::Scissors,
            ("X", Move::Paper) => Move::Rock,
            ("X", Move::Scissors) => Move::Paper,
            ("Y", Move::Rock) => Move::Rock,
            ("Y", Move::Paper) => Move::Paper,
            ("Y", Move::Scissors) => Move::Scissors,
            ("Z", Move::Rock) => Move::Paper,
            ("Z", Move::Paper) => Move::Scissors,
            ("Z", Move::Scissors) => Move::Rock,
            _ => unreachable!(),
        }
    }
}

fn games_1(input: &str) -> IResult<&str, Vec<Game>> {
    many0(terminated(game_1, opt(char('\n'))))(input)
}

fn games_2(input: &str) -> IResult<&str, Vec<Game>> {
    many0(terminated(game_2, opt(char('\n'))))(input)
}

fn game_1(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(parse_move, char(' '), parse_move),
        Game::from,
    )(input)
}

fn game_2(input: &str) -> IResult<&str, Game> {
    let (input, opponent_move) = terminated(parse_move, char(' '))(input)?;
    let (input, own_move) = map(take(1usize), |input| {
        Move::from_answer(input, &opponent_move)
    })(input)?;

    Ok((input, (opponent_move, own_move).into()))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(take(1usize), Move::from_move)(input)
}
