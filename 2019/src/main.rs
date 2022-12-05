use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

type ProblemFns = (fn(&str) -> String, fn(&str) -> String);

const DAYS: [ProblemFns; 0] = [
    //(day01::part_1, day01::part_2),
    //(day02::part_1, day02::part_2),
    //(day03::part_1, day03::part_2),
    //(day04::part_1, day04::part_2),
    //(day05::part_1, day05::part_2),
    //(day06::part_1, day06::part_2),
    //(day07::part_1, day07::part_2),
    //(day08::part_1, day08::part_2),
    //(day09::part_1, day09::part_2),
    //(day10::part_1, day10::part_2),
    //(day11::part_1, day11::part_2),
    //(day12::part_1, day12::part_2),
    //(day13::part_1, day13::part_2),
    //(day14::part_1, day14::part_2),
    //(day15::part_1, day15::part_2),
    //(day16::part_1, day16::part_2),
    //(day17::part_1, day17::part_2),
    //(day18::part_1, day18::part_2),
    //(day19::part_1, day19::part_2),
    //(day20::part_1, day20::part_2),
    //(day21::part_1, day21::part_2),
    //(day22::part_1, day22::part_2),
    //(day23::part_1, day23::part_2),
    //(day24::part_1, day24::part_2),
    //(day25::part_1, day25::part_2),
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
