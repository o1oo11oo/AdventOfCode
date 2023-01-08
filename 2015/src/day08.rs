pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let mut bytes = l.as_bytes().iter().skip(1);
            let mut count = 0;
            while let Some(c) = bytes.next() {
                if c == &b'\\' && bytes.next() == Some(&b'x') {
                    bytes.next();
                    bytes.next();
                }
                count += 1;
            }
            l.as_bytes().len() - count + 1
        })
        .sum::<usize>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'\\' => 2,
                    b'"' => 2,
                    _ => 1,
                })
                .sum::<usize>()
                + 2
                - l.as_bytes().len()
        })
        .sum::<usize>()
        .to_string()
}
