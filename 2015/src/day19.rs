use itertools::Itertools;
use regex::Regex;

pub(crate) fn part_1(input: &str) -> String {
    let (replacements, molecule) = get_input(input);
    neighbours(molecule, &replacements).count().to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (replacements, molecule) = get_input(input);

    let mut remaining = molecule.to_owned();
    remaining = remaining.replace("Rn", "(");
    remaining = remaining.replace('Y', ",");
    remaining = remaining.replace("Ar", ")");
    for el in get_elements(&replacements) {
        remaining = remaining.replace(&el, "0");
    }

    let mut remaining = remaining
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'(' => Value::Start,
            b',' => Value::Sep,
            b')' => Value::End,
            v => Value::Num(v - b'0'),
        })
        .collect_vec();

    // Alternatively just count(NumSymbols) - count(Rn) - count(Ar) - 2 * count(Y) - 1
    while remaining.len() > 1 {
        remaining = fold(remaining);
    }

    remaining[0].value().unwrap().to_string()
}

fn neighbours<'a>(
    molecule: &'a str,
    replacements: &'a [(&str, &str)],
) -> impl Iterator<Item = String> + 'a {
    replacements
        .iter()
        .copied()
        .flat_map(|(s, r)| {
            molecule
                .match_indices(s)
                .map(|(i, _)| molecule[0..i].to_owned() + r + &molecule[i + s.len()..])
        })
        .sorted()
        .dedup()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Value {
    Num(u8),
    Start,
    Sep,
    End,
}

impl Value {
    fn value(&self) -> Option<u8> {
        match self {
            Self::Num(v) => Some(*v),
            _ => None,
        }
    }
}

fn fold(items: Vec<Value>) -> Vec<Value> {
    let items = items
        .into_iter()
        .coalesce(|a, b| match (a, b) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b + 1)),
            _ => Err((a, b)),
        })
        .collect_vec();

    let mut new = vec![];
    let mut it = items.iter().enumerate();

    while let Some((i, _)) = it.next() {
        if i + 3 < items.len() && items[i + 1] == Value::Start && items[i + 3] == Value::End {
            if let Value::Num(v1) = items[i] {
                if let Value::Num(v2) = items[i + 2] {
                    new.push(Value::Num(v1 + v2 + 1));
                    it = it.dropping(3);
                    continue;
                }
            }
        }

        if i + 5 < items.len()
            && items[i + 1] == Value::Start
            && items[i + 3] == Value::Sep
            && items[i + 5] == Value::End
        {
            if let Value::Num(v1) = items[i] {
                if let Value::Num(v2) = items[i + 2] {
                    if let Value::Num(v3) = items[i + 4] {
                        new.push(Value::Num(v1 + v2 + v3 + 1));
                        it = it.dropping(5);
                        continue;
                    }
                }
            }
        }

        if i + 7 < items.len()
            && items[i + 1] == Value::Start
            && items[i + 3] == Value::Sep
            && items[i + 5] == Value::Sep
            && items[i + 7] == Value::End
        {
            if let Value::Num(v1) = items[i] {
                if let Value::Num(v2) = items[i + 2] {
                    if let Value::Num(v3) = items[i + 4] {
                        if let Value::Num(v4) = items[i + 6] {
                            new.push(Value::Num(v1 + v2 + v3 + v4 + 1));
                            it = it.dropping(7);
                            continue;
                        }
                    }
                }
            }
        }

        new.push(items[i]);
    }
    new
}

fn get_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (replacements, molecule) = input.trim().split_once("\n\n").unwrap();
    let replacements = replacements
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .collect();
    (replacements, molecule)
}

fn get_elements(replacements: &[(&str, &str)]) -> Vec<String> {
    let el_regex = Regex::new(r"[A-Z][a-z]?").unwrap();
    replacements
        .iter()
        .flat_map(|r| [r.0, r.1])
        .flat_map(|r| {
            el_regex
                .find_iter(r)
                .map(|m| m.as_str().to_owned())
                .collect_vec()
        })
        .filter(|el| !["Rn", "Y", "Ar"].contains(&el.as_str()))
        .sorted_by(|a, b| match b.len().cmp(&a.len()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.cmp(b),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        })
        .dedup()
        .collect()
}
