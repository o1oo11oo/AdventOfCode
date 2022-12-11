pub(crate) fn part_1(input: &str) -> String {
    let instructions =
        std::iter::once(Instruction::Noop).chain(input.lines().flat_map(Instruction::from));
    const INTERESTING_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut interesting_values = vec![];
    let mut x = 1;

    for (cycles, ins) in instructions.enumerate() {
        for c in INTERESTING_CYCLES {
            if c == cycles {
                interesting_values.push(c as i32 * x);
            }
        }

        if let Instruction::Addx(val) = ins {
            x += val;
        }
    }

    interesting_values.iter().sum::<i32>().to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    const DISPLAY_WIDTH: usize = 40;
    let instructions = input.lines().flat_map(Instruction::from);
    let mut x: i32 = 1;
    let mut display = Vec::new();

    for (cycle, ins) in instructions.enumerate() {
        display.push(
            if x.abs_diff((cycle % DISPLAY_WIDTH).try_into().unwrap()) <= 1 {
                '#'
            } else {
                '.'
            },
        );

        if let Instruction::Addx(val) = ins {
            x += val;
        }
    }

    std::iter::once("".to_owned())
        .chain(display.chunks(DISPLAY_WIDTH).map(|c| c.iter().collect()))
        .collect::<Vec<String>>()
        .join("\n")
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from(value: &str) -> Vec<Self> {
        let mut split = value.split_ascii_whitespace();

        match split.next().unwrap() {
            "noop" => vec![Instruction::Noop],
            "addx" => vec![
                Instruction::Noop,
                Instruction::Addx(split.next().unwrap().parse().unwrap()),
            ],
            _ => unreachable!(),
        }
    }
}
