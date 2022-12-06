use std::collections::{hash_map::RandomState, HashSet};

pub(crate) fn part_1(input: &str) -> String {
    let (count, _) = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|&(_, win)| {
            win[0] != win[1]
                && win[0] != win[2]
                && win[0] != win[3]
                && win[1] != win[2]
                && win[1] != win[3]
                && win[2] != win[3]
        })
        .unwrap();

    (count + 4).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (count, _) = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|&(_, win)| HashSet::<_, RandomState>::from_iter(win).len() == 14)
        .unwrap();

    (count + 14).to_string()
}
