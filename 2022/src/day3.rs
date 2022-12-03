pub(crate) fn part_1(input: &str) -> String {
    let duplicates = input
        .lines()
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            let first = first
                .chars()
                .map(char_to_bitflag)
                .reduce(|a, i| a | i)
                .unwrap();
            let second = second
                .chars()
                .map(char_to_bitflag)
                .reduce(|a, i| a | i)
                .unwrap();

            let duplicate = first & second;
            (duplicate, bitflag_to_priority(duplicate))
        })
        .collect::<Vec<_>>();

    let alphabet = ('A'..='Z')
        .rev()
        .chain(('a'..='z').rev())
        .collect::<String>();
    log::info!("{alphabet: >64}");
    for (idx, prio) in &duplicates {
        log::info!("{idx:#064b}, prio: {prio}");
    }

    duplicates
        .iter()
        .map(|(_, prio)| prio.to_owned())
        .reduce(|a, i| a + i)
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let badges = input
        .lines()
        .array_chunks() // needs rust nightly
        .map(|chunk: [&str; 3]| {
            chunk
                .iter()
                .map(|line| {
                    line.chars()
                        .map(char_to_bitflag)
                        .reduce(|a, i| a | i)
                        .unwrap()
                })
                .reduce(|a, i| a & i)
                .unwrap()
        })
        .map(|b| (b, bitflag_to_priority(b)))
        .collect::<Vec<_>>();

    let alphabet = ('A'..='Z')
        .rev()
        .chain(('a'..='z').rev())
        .collect::<String>();
    log::info!("{alphabet: >64}");
    for (idx, prio) in &badges {
        log::info!("{idx:#064b}, prio: {prio}");
    }

    badges
        .iter()
        .map(|(_, prio)| prio.to_owned())
        .reduce(|a, i| a + i)
        .unwrap()
        .to_string()
}

fn char_to_bitflag(ch: char) -> u64 {
    const A_LOWER_DIGIT: u64 = 'a' as u64;
    const A_UPPER_DIGIT: u64 = 'A' as u64;
    let ch_digit = ch as u64;
    log::debug!("ch_digit: {ch_digit}");

    let idx: u64 = match ch {
        'a'..='z' => ch_digit - A_LOWER_DIGIT,
        'A'..='Z' => ch_digit - A_UPPER_DIGIT + 26,
        _ => unreachable!(),
    };
    log::debug!("idx: {idx}");

    1 << idx
}

fn bitflag_to_priority(mut bit: u64) -> u32 {
    let mut count = 0;

    while bit > 0 {
        bit >>= 1;
        count += 1;
    }

    count
}
