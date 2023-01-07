use itertools::Itertools;
use parse_display::FromStr;

pub(crate) fn part_1(input: &str) -> String {
    let mut lights = vec![vec![false; 1000]; 1000];
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .for_each(|action| {
            match action {
                Action::TurnOn(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] = true;
                }),
                Action::Toggle(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] = !lights[row][col];
                }),
                Action::TurnOff(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] = false;
                }),
            };
        });

    lights
        .iter()
        .flat_map(|row| row.iter())
        .filter(|l| **l)
        .count()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut lights = vec![vec![0_u32; 1000]; 1000];
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .for_each(|action| {
            match action {
                Action::TurnOn(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] += 1;
                }),
                Action::Toggle(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] += 2;
                }),
                Action::TurnOff(_, _, _, _) => action.range().for_each(|(row, col)| {
                    lights[row][col] = lights[row][col].saturating_sub(1);
                }),
            };
        });

    lights
        .iter()
        .flat_map(|row| row.iter())
        .sum::<u32>()
        .to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, FromStr)]
enum Action {
    #[display("turn on {0},{2} through {1},{3}")]
    TurnOn(usize, usize, usize, usize),
    #[display("toggle {0},{2} through {1},{3}")]
    Toggle(usize, usize, usize, usize),
    #[display("turn off {0},{2} through {1},{3}")]
    TurnOff(usize, usize, usize, usize),
}

impl Action {
    fn range(&self) -> impl Iterator<Item = (usize, usize)> {
        match self {
            Action::TurnOn(row_from, row_to, col_from, col_to) => {
                (*row_from..=*row_to).cartesian_product(*col_from..=*col_to)
            }
            Action::Toggle(row_from, row_to, col_from, col_to) => {
                (*row_from..=*row_to).cartesian_product(*col_from..=*col_to)
            }
            Action::TurnOff(row_from, row_to, col_from, col_to) => {
                (*row_from..=*row_to).cartesian_product(*col_from..=*col_to)
            }
        }
    }
}
