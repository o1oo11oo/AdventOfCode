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

fn get_scenic_score(forest: &[Vec<u8>], row_idx: usize, column_idx: usize) -> u32 {
    let height = forest[row_idx][column_idx];

    let mut upper_score = 0;
    for i in (0..row_idx).rev() {
        upper_score += 1;
        if forest[i][column_idx] >= height {
            break;
        }
    }

    let mut lower_score = 0;
    #[allow(clippy::needless_range_loop)]
    for i in row_idx + 1..forest.len() {
        lower_score += 1;
        if forest[i][column_idx] >= height {
            break;
        }
    }

    let mut left_score = 0;
    for i in (0..column_idx).rev() {
        left_score += 1;
        if forest[row_idx][i] >= height {
            break;
        }
    }

    let mut right_score = 0;
    for i in column_idx + 1..forest[row_idx].len() {
        right_score += 1;
        if forest[row_idx][i] >= height {
            break;
        }
    }

    upper_score * lower_score * left_score * right_score
}
