pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|l| str::parse::<u32>(l).ok())
        .map(|l| l / 3 - 2)
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .filter_map(|l| str::parse::<u32>(l).ok())
        .map(calculate_fuel_per_module)
        .sum::<u32>()
        .to_string()
}

fn calculate_fuel_per_module(module: u32) -> u32 {
    let mut sum = 0;
    let mut fuel_weight = module / 3 - 2;
    while fuel_weight > 0 {
        sum += fuel_weight;
        fuel_weight = (fuel_weight / 3).saturating_sub(2);
    }

    sum
}
