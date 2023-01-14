use std::borrow::Cow;

use itertools::Itertools;

pub(crate) fn part_1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c == b'#').collect_vec())
        .collect_vec();
    log::debug!("grid:\n{}", display_gol_grid(&grid));

    itertools::iterate(grid, |g| step(g, false))
        .skip(1)
        .take(100)
        .last()
        .unwrap()
        .iter()
        .map(|row| row.iter().filter(|c| **c).count())
        .sum::<usize>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&c| c == b'#').collect_vec())
        .collect_vec();
    let row_len = grid.len();
    let col_len = grid[0].len();
    grid[0][0] = true;
    grid[0][col_len - 1] = true;
    grid[row_len - 1][0] = true;
    grid[row_len - 1][col_len - 1] = true;

    itertools::iterate(grid, |g| step(g, true))
        .skip(1)
        .take(100)
        .last()
        .unwrap()
        .iter()
        .map(|row| row.iter().filter(|c| **c).count())
        .sum::<usize>()
        .to_string()
}

fn step(grid: &[Vec<bool>], corner_correction: bool) -> Vec<Vec<bool>> {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut next = vec![vec![false; col_len]; row_len];

    if corner_correction {
        next[0][0] = true;
        next[0][col_len - 1] = true;
        next[row_len - 1][0] = true;
        next[row_len - 1][col_len - 1] = true;
    }

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let count = neighbours((row, col), row_len, col_len)
                .filter(|pos| grid[pos.0][pos.1])
                .count();
            if count == 3 || (count == 2 && grid[row][col]) {
                next[row][col] = true;
            }
        }
    }
    next
}

fn neighbours(
    pos: (usize, usize),
    row_len: usize,
    col_len: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let rows = [pos.0.saturating_sub(1), pos.0, pos.0 + 1]
        .into_iter()
        .dedup()
        .filter(move |&row| row < row_len);
    let cols = [pos.1.saturating_sub(1), pos.1, pos.1 + 1]
        .into_iter()
        .dedup()
        .filter(move |&col| col < col_len);
    rows.cartesian_product(cols).filter(move |&p| p != pos)
}

fn display_gol_grid(grid: &[Vec<bool>]) -> String {
    display_grid_with_map(grid, |&t| if t { "#".into() } else { ".".into() })
}

fn display_grid_with_map<T>(grid: &[Vec<T>], map: fn(&T) -> Cow<'static, str>) -> String {
    grid.iter()
        .map(|row| row.iter().map(map).join(""))
        .join("\n")
}
