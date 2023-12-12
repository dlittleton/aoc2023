use std::collections::BTreeSet;
use std::iter::repeat;

use aoc2023::{collections::grid::Grid, util::combinations};

aoc2023::solver!(part1, part2);

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

fn part2(lines: &[String]) -> String {
    let spacing = 1000000;
    let grid: Grid<_> = lines.iter().map(|line| line.chars()).collect();

    let empty_cols: BTreeSet<_> = get_empty_cols(&grid).into_iter().collect();
    let empty_rows: BTreeSet<_> = get_empty_rows(&grid).into_iter().collect();

    let galaxies: Vec<_> = grid
        .enumerate()
        .filter_map(|(i, j, c)| match *c {
            '#' => Some((i, j)),
            _ => None,
        })
        .collect();

    let total: usize = combinations(&galaxies)
        .map(|(a, b)| {
            let min_row = a.0.min(b.0);
            let max_row = a.0.max(b.0);

            let row_count =
                (max_row - min_row) + (empty_rows.range(min_row..max_row).count() * (spacing - 1));

            let min_col = a.1.min(b.1);
            let max_col = a.1.max(b.1);

            let col_count =
                (max_col - min_col) + (empty_cols.range(min_col..max_col).count() * (spacing - 1));
            row_count + col_count
        })
        .sum();

    format!("{}", total)
}

fn expand_grid(grid: &mut Grid<char>) {
    let empty_rows = get_empty_rows(grid);

    empty_rows
        .iter()
        .rev()
        .for_each(|i| grid.insert_row_at(*i, repeat('.')));

    let empty_cols = get_empty_cols(grid);

    empty_cols
        .iter()
        .rev()
        .for_each(|i| grid.insert_col_at(*i, repeat('.')));
}

fn get_empty_rows(grid: &Grid<char>) -> Vec<usize> {
    grid.row_wise_iter()
        .enumerate()
        .filter_map(|(i, mut row)| match row.all(|c| c == &'.') {
            true => Some(i),
            _ => None,
        })
        .collect()
}

fn get_empty_cols(grid: &Grid<char>) -> Vec<usize> {
    grid.col_wise_iter()
        .enumerate()
        .filter_map(|(i, mut col)| match col.all(|c| c == &'.') {
            true => Some(i),
            _ => None,
        })
        .collect()
}
