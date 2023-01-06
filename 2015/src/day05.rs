use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .filter(contains_three_vowels)
        .filter(contains_duplicate_letter)
        .filter(not_contains_naughty)
        .count()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .filter(contains_separated_pair)
        .filter(contains_duplicate_with_separator)
        .count()
        .to_string()
}

fn contains_three_vowels(input: &&str) -> bool {
    input
        .as_bytes()
        .iter()
        .filter(|&c| c == &b'a' || c == &b'e' || c == &b'i' || c == &b'o' || c == &b'u')
        .count()
        >= 3
}

fn contains_duplicate_letter(input: &&str) -> bool {
    input.as_bytes().iter().tuple_windows().any(|(a, b)| a == b)
}

fn not_contains_naughty(input: &&str) -> bool {
    input.as_bytes().iter().tuple_windows().all(|(a, b)| {
        !matches!(
            (a, b),
            (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y')
        )
    })
}

fn contains_separated_pair(input: &&str) -> bool {
    input
        .as_bytes()
        .windows(2)
        .enumerate()
        .any(|(needle_idx, needle_val)| {
            input
                .as_bytes()
                .windows(2)
                .skip(needle_idx + 2)
                .any(|haystack_val| needle_val == haystack_val)
        })
}

fn contains_duplicate_with_separator(input: &&str) -> bool {
    input
        .as_bytes()
        .iter()
        .tuple_windows()
        .any(|(a, _, b)| a == b)
}
