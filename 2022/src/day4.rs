pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(get_numbers_from_line)
        .map(contains_range)
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .map(get_numbers_from_line)
        .map(intersects_range)
        .sum::<u32>()
        .to_string()
}

fn get_numbers_from_line(line: &str) -> [u32; 4] {
    let mut parts = line.split(',').flat_map(|part| part.split('-'));
    let left_start: u32 = parts.next().unwrap().parse().unwrap();
    let left_end: u32 = parts.next().unwrap().parse().unwrap();
    let right_start: u32 = parts.next().unwrap().parse().unwrap();
    let right_end: u32 = parts.next().unwrap().parse().unwrap();

    [left_start, left_end, right_start, right_end]
}

fn contains_range(numbers: [u32; 4]) -> u32 {
    (numbers[0] <= numbers[2] && numbers[1] >= numbers[3]
        || numbers[0] >= numbers[2] && numbers[1] <= numbers[3]) as u32
}

fn intersects_range(numbers: [u32; 4]) -> u32 {
    let left_range = numbers[0]..=numbers[1];
    let right_range = numbers[2]..=numbers[3];

    (left_range.contains(&numbers[2])
        || left_range.contains(&numbers[3])
        || right_range.contains(&numbers[0])
        || right_range.contains(&numbers[1])) as u32
}
