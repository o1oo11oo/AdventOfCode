pub(crate) fn part_1(input: &str) -> String {
    let password: [u8; 8] = input.trim().as_bytes().try_into().unwrap();
    let next = itertools::iterate(password, |p| inc_password(*p))
        .find(valid_password)
        .unwrap();
    String::from_utf8(next.to_vec()).unwrap()
}

pub(crate) fn part_2(input: &str) -> String {
    let password: [u8; 8] = input.trim().as_bytes().try_into().unwrap();
    let next = itertools::iterate(password, |p| inc_password(*p))
        .filter(valid_password)
        .nth(1)
        .unwrap();
    String::from_utf8(next.to_vec()).unwrap()
}

fn inc_password(mut password: [u8; 8]) -> [u8; 8] {
    let mut idx = 7;
    loop {
        password[idx] += 1;
        if password[idx] > b'z' {
            password[idx] = b'a';
            idx = idx.checked_sub(1).unwrap_or(7);
        } else {
            break;
        }
    }
    password
}

fn valid_password(password: &[u8; 8]) -> bool {
    let increasing = password
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let none_forbidden = password
        .iter()
        .all(|c| c != &b'i' && c != &b'o' && c != &b'l');
    let non_overlapping_pairs = password
        .windows(2)
        .enumerate()
        .any(|(first_idx, first_val)| {
            first_val[0] == first_val[1]
                && password
                    .windows(2)
                    .skip(first_idx + 2)
                    .any(|second_val| first_val != second_val && second_val[0] == second_val[1])
        });

    increasing && none_forbidden && non_overlapping_pairs
}
