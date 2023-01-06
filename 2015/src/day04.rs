pub(crate) fn part_1(input: &str) -> String {
    (0..)
        .find(|num| {
            let phrase = format!("{}{num}", input.trim());
            let hash = md5::compute(phrase);
            hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0
        })
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    (0..)
        .find(|num| {
            let phrase = format!("{}{num}", input.trim());
            let hash = md5::compute(phrase);
            hash[0] == 0 && hash[1] == 0 && hash[2] == 0
        })
        .unwrap()
        .to_string()
}
