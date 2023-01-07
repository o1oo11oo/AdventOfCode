use rayon::prelude::*;

pub(crate) fn part_1(input: &str) -> String {
    (0_u64..)
        .step_by(250_000)
        .find_map(|bound| {
            (bound..bound + 250_000).into_par_iter().find_first(|num| {
                let phrase = format!("{}{num}", input.trim());
                let hash = md5::compute(phrase);
                hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0
            })
        })
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    (0_u64..)
        .step_by(1_000_000)
        .find_map(|bound| {
            (bound..bound + 1_000_000)
                .into_par_iter()
                .find_first(|num| {
                    let phrase = format!("{}{num}", input.trim());
                    let hash = md5::compute(phrase);
                    hash[0] == 0 && hash[1] == 0 && hash[2] == 0
                })
        })
        .unwrap()
        .to_string()
}
