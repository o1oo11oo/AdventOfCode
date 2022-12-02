use nom::{
    bytes::complete::take,
    character::complete::char,
    combinator::{map, map_res, opt},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    let example =
        std::fs::read_to_string("example.txt").expect("Should have been able to read the file");
    let input =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    part_1(&input);
}

fn part_1(input: &str) {
    let (remaining, games) = games(input).unwrap();
    log::debug!("{games:#?}");
    let scores = games.iter().map(|g| g.score()).collect::<Vec<_>>();
    log::debug!("scores: {scores:?}");
    let total: u32 = scores.iter().sum();
    log::info!("total score: {total}");
}

#[derive(Debug)]
struct Game {
    opponent_move: Move,
    own_move: Move,
}

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

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

fn games(input: &str) -> IResult<&str, Vec<Game>> {
    many0(terminated(game, opt(char('\n'))))(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(parse_move, char(' '), parse_move),
        Game::from,
    )(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map_res(take(1usize), Move::try_from)(input)
}
