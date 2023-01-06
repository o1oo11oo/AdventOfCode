use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let r = l
                .split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .tuple_combinations()
                .map(|(s1, s2)| s1 * s2)
                .fold((u32::MAX, 0), |acc, curr| {
                    (acc.0.min(curr), acc.1 + 2 * curr)
                });
            r.0 + r.1
        })
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let iter = l.split('x').map(|n| n.parse::<u32>().unwrap());
            iter.clone()
                .tuple_combinations()
                .map(|(s1, s2)| 2 * (s1 + s2))
                .min()
                .unwrap()
                + iter.product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}
