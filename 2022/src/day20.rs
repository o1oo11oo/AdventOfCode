type Num = i64;

pub(crate) fn part_1(input: &str) -> String {
    let mut file = input
        .lines()
        .enumerate()
        .map(MoveStatus::from)
        .collect::<Vec<_>>();
    decryption_round(&mut file);
    get_coordinates(&file).to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let decryption_key = 811589153;
    let mut file = input
        .lines()
        .enumerate()
        .map(MoveStatus::from)
        .map(|status| status.apply_key(decryption_key))
        .collect::<Vec<_>>();
    for _ in 0..10 {
        decryption_round(&mut file);
        file.iter_mut().for_each(|status| *status = status.reset());
    }
    get_coordinates(&file).to_string()
}

fn decryption_round(file: &mut Vec<MoveStatus>) {
    let len = file.len() as Num - 1;
    for order in 0..file.len() {
        let idx = file
            .iter()
            .enumerate()
            .find(|(_, status)| status.order() == order)
            .unwrap()
            .0;

        let MoveStatus::Unmoved { value, order } = file[idx] else {
            unreachable!()
        };
        let new_idx = (idx as Num + value).rem_euclid(len).try_into().unwrap();

        if idx == new_idx {
            file[idx] = MoveStatus::Moved { value, order }
        } else {
            file.remove(idx);
            file.insert(new_idx, MoveStatus::Moved { value, order });
        }
    }
}

fn get_coordinates(file: &[MoveStatus]) -> Num {
    let start = file
        .iter()
        .enumerate()
        .find_map(|(idx, status)| if status.value() == 0 { Some(idx) } else { None })
        .unwrap();
    let pos = [
        (start + 1000).rem_euclid(file.len()),
        (start + 2000).rem_euclid(file.len()),
        (start + 3000).rem_euclid(file.len()),
    ];

    pos.iter().map(|idx| file[*idx].value()).sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MoveStatus {
    Unmoved { value: Num, order: usize },
    Moved { value: Num, order: usize },
}

impl MoveStatus {
    fn value(&self) -> Num {
        match self {
            MoveStatus::Unmoved { value: v, order: _ } => *v,
            MoveStatus::Moved { value: v, order: _ } => *v,
        }
    }

    fn order(&self) -> usize {
        match self {
            MoveStatus::Unmoved { value: _, order: o } => *o,
            MoveStatus::Moved { value: _, order: o } => *o,
        }
    }

    fn reset(self) -> Self {
        MoveStatus::Unmoved {
            value: self.value(),
            order: self.order(),
        }
    }

    fn apply_key(self, key: Num) -> Self {
        match self {
            MoveStatus::Unmoved { value, order } => MoveStatus::Unmoved {
                value: value * key,
                order,
            },
            MoveStatus::Moved { value, order } => MoveStatus::Moved {
                value: value * key,
                order,
            },
        }
    }
}

impl From<(usize, &str)> for MoveStatus {
    fn from((order, value): (usize, &str)) -> Self {
        MoveStatus::Unmoved {
            value: value.parse().unwrap(),
            order,
        }
    }
}
