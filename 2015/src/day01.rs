pub(crate) fn part_1(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .enumerate()
        .fold((None, 0), |mut acc, curr| {
            acc.1 += curr.1;
            if acc.1 == -1 && acc.0.is_none() {
                acc.0 = Some(curr.0 + 1)
            }
            acc
        })
        .0
        .unwrap()
        .to_string()
}
