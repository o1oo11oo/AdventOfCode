pub(crate) fn part_1(input: &str) -> String {
    let mut forest = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| (c - b'0').into())
                .collect::<Vec<Visible>>()
        })
        .collect::<Vec<_>>();

    traverse_horizontally(&mut forest);
    traverse_vertically(&mut forest);

    forest
        .iter()
        .flat_map(|r| r.iter())
        .filter(|tree| matches!(tree, Visible::True(_)))
        .count()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let forest = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| (c - b'0')).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut scores = vec![vec![0; forest[0].len()]; forest.len()];
    for row_idx in 0..forest.len() {
        for column_idx in 0..forest[row_idx].len() {
            scores[row_idx][column_idx] = get_scenic_score(&forest, row_idx, column_idx);
        }
    }

    scores
        .iter()
        .flat_map(|r| r.iter())
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug)]
enum Visible {
    True(i8),
    False(i8),
}

impl Visible {
    fn height(&self) -> i8 {
        match self {
            Visible::True(height) => *height,
            Visible::False(height) => *height,
        }
    }
}

impl From<u8> for Visible {
    fn from(value: u8) -> Self {
        Visible::False(value as i8)
    }
}

fn traverse_horizontally(forest: &mut [Vec<Visible>]) {
    let mut current_highest;
    for row in forest.iter_mut() {
        current_highest = -1i8;
        for tree in row.iter_mut() {
            handle_tree(tree, &mut current_highest);
        }
    }

    for row in forest.iter_mut() {
        current_highest = -1i8;
        for tree in row.iter_mut().rev() {
            handle_tree(tree, &mut current_highest);
        }
    }
}

fn traverse_vertically(forest: &mut [Vec<Visible>]) {
    let mut current_highest;
    for column_idx in 0..forest[0].len() {
        current_highest = -1i8;
        for row in forest.iter_mut() {
            handle_tree(&mut row[column_idx], &mut current_highest);
        }
    }

    for column_idx in 0..forest[0].len() {
        current_highest = -1i8;
        for row in forest.iter_mut().rev() {
            handle_tree(&mut row[column_idx], &mut current_highest);
        }
    }
}

fn handle_tree(tree: &mut Visible, current_highest: &mut i8) {
    let height = tree.height();

    if height > *current_highest {
        *tree = Visible::True(height);
        *current_highest = height;
    }
}

fn get_scenic_score(forest: &[Vec<u8>], row_idx: usize, column_idx: usize) -> usize {
    let height = forest[row_idx][column_idx];

    let upper_score = 1 + forest
        .iter()
        .rev()
        .skip(forest.len() - row_idx)
        .take_while(|row| row[column_idx] < height)
        .count();

    let lower_score = forest
        .iter()
        .skip(row_idx + 1)
        .take_while(|row| row[column_idx] < height)
        .count();

    let row = &forest[row_idx];
    let left_score = row
        .iter()
        .rev()
        .skip(row.len() - column_idx)
        .take_while(|&&tree| tree < height)
        .count();

    let right_score = row
        .iter()
        .skip(column_idx + 1)
        .take_while(|&&tree| tree < height)
        .count();

    upper_score * lower_score * left_score * right_score
}
