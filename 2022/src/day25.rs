#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) fn part_1(input: &str) -> String {
    let numbers = input.lines().map(snafu_to_dec).collect::<Vec<_>>();
    let sum = numbers.iter().sum::<i128>();

    log::debug!("{numbers:?}");
    log::debug!("sum: {sum}");

    dec_to_snafu(sum)
}

pub(crate) fn part_2(input: &str) -> String {
    "Not implemented!".to_string()
}

fn snafu_to_dec(num: &str) -> i128 {
    num.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .fold(0i128, |acc, (exponent, char)| {
            5i128.pow(exponent as u32) * char_to_num(char) + acc
        })
}

fn dec_to_snafu(mut num: i128) -> String {
    let mut res = vec![];

    while num > 0 {
        res.push(num % 5);
        num /= 5;
    }

    res.push(0);
    for i in 0..res.len() - 1 {
        if res[i] == 3 {
            res[i] = -2;
            res[i + 1] += 1;
        } else if res[i] == 4 {
            res[i] = -1;
            res[i + 1] += 1;
        } else if res[i] == 5 {
            res[i] = 0;
            res[i + 1] += 1;
        }
    }

    if let Some(&0) = res.last() {
        res.pop();
    }

    res.into_iter().rev().map(num_to_char).collect()
}

fn char_to_num(char: &u8) -> i128 {
    match char {
        b'=' => -2,
        b'-' => -1,
        b'0' | b'1' | b'2' => *char as i128 - b'0' as i128,
        _ => unreachable!(),
    }
}

fn num_to_char(num: i128) -> char {
    match num {
        -2 => '=',
        -1 => '-',
        0 | 1 | 2 => (b'0' + num as u8) as char,
        _ => unreachable!(),
    }
}
