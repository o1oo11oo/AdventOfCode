use std::collections::HashSet;

pub(crate) fn part_1(input: &str) -> String {
    input
        .trim()
        .as_bytes()
        .iter()
        .fold(((0, 0), HashSet::from([(0, 0)])), |mut acc, curr| {
            let new = match curr {
                b'^' => (acc.0 .0, acc.0 .1 - 1),
                b'v' => (acc.0 .0, acc.0 .1 + 1),
                b'<' => (acc.0 .0 - 1, acc.0 .1),
                b'>' => (acc.0 .0 + 1, acc.0 .1),
                _ => (acc.0 .0, acc.0 .1),
            };
            acc.1.insert(new);
            (new, acc.1)
        })
        .1
        .len()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .fold(
            ((0, 0), (0, 0), HashSet::from([(0, 0)])),
            |mut acc, curr| {
                let pos = if curr.0 % 2 == 0 {
                    &mut acc.0
                } else {
                    &mut acc.1
                };
                *pos = match curr.1 {
                    b'^' => (pos.0, pos.1 - 1),
                    b'v' => (pos.0, pos.1 + 1),
                    b'<' => (pos.0 - 1, pos.1),
                    b'>' => (pos.0 + 1, pos.1),
                    _ => (pos.0, pos.1),
                };
                acc.2.insert(*pos);
                acc
            },
        )
        .2
        .len()
        .to_string()
}
