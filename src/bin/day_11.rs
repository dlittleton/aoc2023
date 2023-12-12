use std::iter::repeat;

use aoc2023::{collections::grid::Grid, util::combinations};

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let mut grid: Grid<_> = lines.iter().map(|line| line.chars()).collect();

    expand_grid(&mut grid);

    let galaxies: Vec<_> = grid
        .enumerate()
        .filter_map(|(i, j, c)| match *c {
            '#' => Some((i, j)),
            _ => None,
        })
        .collect();

    let total: usize = combinations(&galaxies)
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum();

    format!("{}", total)
}

fn expand_grid(grid: &mut Grid<char>) {
    let empty_rows: Vec<_> = grid
        .row_wise_iter()
        .enumerate()
        .filter_map(|(i, mut row)| match row.all(|c| c == &'.') {
            true => Some(i),
            _ => None,
        })
        .collect();

    empty_rows
        .iter()
        .rev()
        .for_each(|i| grid.insert_row_at(*i, repeat('.')));

    let empty_cols: Vec<_> = grid
        .col_wise_iter()
        .enumerate()
        .filter_map(|(i, mut col)| match col.all(|c| c == &'.') {
            true => Some(i),
            _ => None,
        })
        .collect();

    empty_cols
        .iter()
        .rev()
        .for_each(|i| grid.insert_col_at(*i, repeat('.')));
}
