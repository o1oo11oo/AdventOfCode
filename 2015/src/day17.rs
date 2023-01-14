pub(crate) fn part_1(input: &str) -> String {
    let sizes = input
        .lines()
        .map(|l| l.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let target = if sizes.len() == 5 { 25 } else { 150 };

    find_possibilities(&sizes, target, vec![]).len().to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let sizes = input
        .lines()
        .map(|l| l.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let target = if sizes.len() == 5 { 25 } else { 150 };
    let possibilities = find_possibilities(&sizes, target, vec![]);
    let min = possibilities.iter().map(|p| p.len()).min().unwrap();

    possibilities
        .iter()
        .filter(|p| p.len() == min)
        .count()
        .to_string()
}

fn find_possibilities(sizes: &[u8], target: u8, current: Vec<u8>) -> Vec<Vec<u8>> {
    if target == 0 {
        return vec![current];
    }

    sizes
        .iter()
        .enumerate()
        .filter(|(_, s)| **s <= target)
        .flat_map(|(i, s)| {
            let mut current = current.clone();
            current.push(*s);
            find_possibilities(&sizes[i + 1..], target - s, current)
        })
        .collect()
}
