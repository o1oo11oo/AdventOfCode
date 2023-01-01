type Num = i64;

pub(crate) fn part_1(input: &str) -> String {
    let mut file = input
        .lines()
        .enumerate()
        .map(DecryptionItem::from)
        .collect::<Vec<_>>();
    decryption_round(&mut file);
    get_coordinates(&file).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let decryption_key = 811589153;
    let mut file = input
        .lines()
        .enumerate()
        .map(DecryptionItem::from)
        .map(|item| item.apply_key(decryption_key))
        .collect::<Vec<_>>();
    for _ in 0..10 {
        decryption_round(&mut file);
    }
    get_coordinates(&file).to_string()
}

fn decryption_round(file: &mut Vec<DecryptionItem>) {
    let len = file.len() as Num - 1;
    for order in 0..file.len() {
        let idx = file
            .iter()
            .enumerate()
            .find(|(_, item)| item.order == order)
            .unwrap()
            .0;

        let new_idx = (idx as Num + file[idx].value)
            .rem_euclid(len)
            .try_into()
            .unwrap();

        if idx != new_idx {
            let item = file.remove(idx);
            file.insert(new_idx, item);
        }
    }
}

fn get_coordinates(file: &[DecryptionItem]) -> Num {
    let start = file
        .iter()
        .enumerate()
        .find_map(|(idx, status)| if status.value == 0 { Some(idx) } else { None })
        .unwrap();
    let pos = [
        (start + 1000).rem_euclid(file.len()),
        (start + 2000).rem_euclid(file.len()),
        (start + 3000).rem_euclid(file.len()),
    ];

    pos.iter().map(|idx| file[*idx].value).sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DecryptionItem {
    value: Num,
    order: usize,
}

impl DecryptionItem {
    fn apply_key(self, key: Num) -> Self {
        DecryptionItem {
            value: self.value * key,
            order: self.order,
        }
    }
}

impl From<(usize, &str)> for DecryptionItem {
    fn from((order, value): (usize, &str)) -> Self {
        DecryptionItem {
            value: value.parse().unwrap(),
            order,
        }
    }
}
