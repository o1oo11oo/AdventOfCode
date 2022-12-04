#![feature(iter_array_chunks)]

use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;

type ProblemFns = (fn(&str) -> String, fn(&str) -> String);

const DAYS: [ProblemFns; 4] = [
    (day1::part_1, day1::part_2),
    (day2::part_1, day2::part_2),
    (day3::part_1, day3::part_2),
    (day4::part_1, day4::part_2),
];

#[derive(Parser, Debug)]
struct Args {
    /// The day which problem to run
    #[arg(short, long, default_value_t = DAYS.len() as u8, value_parser = clap::value_parser!(u8).range(1..=(DAYS.len() as i64)))]
    day: u8,

    /// Use the example instead of the full input
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let args = Args::parse();
    let day_idx: usize = args.day.saturating_sub(1).into();
    let (part_1, part_2) = DAYS[day_idx];
    let input_path = format!("input/day{}_", args.day)
        + if args.example {
            "example.txt"
        } else {
            "input.txt"
        };
    let input =
        std::fs::read_to_string(input_path).expect("Should have been able to read the file");

    log::info!("Selected day {}", args.day);
    log::info!("Running part 1...");
    let res = (part_1)(&input);
    log::info!("Done, Result: {res}");

    log::info!("Running part 2...");
    let res = (part_2)(&input);
    log::info!("Done, Result: {res}");
}
