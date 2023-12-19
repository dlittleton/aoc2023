use log::info;
use regex::Regex;
use std::{collections::HashMap, iter::repeat};

aoc2023::solver!(part1);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

type ColorMap = HashMap<Position, String>;
type TrenchGrid = Vec<Vec<char>>;

fn part1(lines: &[String]) -> String {
    let path = parse_path(lines);
    info!("Path length is {}", path.len());
    let mut grid = to_grid(&path);
    fill(&mut grid);

    let total = count_dug(&grid);

    format!("{}", total)
}

fn parse_path(lines: &[String]) -> ColorMap {
    let re_path = Regex::new(r"([RDLU]) (\d+) \((#\w{6})\)").unwrap();

    let mut result = ColorMap::new();

    let mut row = 0;
    let mut col = 0;

    for line in lines {
        for c in re_path.captures_iter(line) {
            let dir = c.get(1).unwrap().as_str();
            let count = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let value = c.get(3).unwrap().as_str();

            for _ in 1..=count {
                match dir {
                    "R" => col += 1,
                    "L" => col -= 1,
                    "U" => row -= 1,
                    "D" => row += 1,
                    _ => panic!("Unexpected direction {}", dir),
                };

                result.insert(Position(row, col), String::from(value));
            }
        }
    }

    result
}

fn to_grid(path: &ColorMap) -> TrenchGrid {
    let min_row = path.keys().map(|k| k.0).min().unwrap();
    let max_row = path.keys().map(|k| k.0).max().unwrap();
    let min_col = path.keys().map(|k| k.1).min().unwrap();
    let max_col = path.keys().map(|k| k.1).max().unwrap();

    let row_range = max_row + 1 - min_row;
    let col_range = max_col + 1 - min_col;

    let mut result: TrenchGrid = (0..row_range)
        .map(|_| repeat('.').take(col_range as usize).collect())
        .collect();

    for k in path.keys() {
        let row_pos = k.0 - min_row;
        let col_pos = k.1 - min_col;

        result[row_pos as usize][col_pos as usize] = '#'
    }

    result
}

fn fill(grid: &mut TrenchGrid) {
    let mut to_visit: Vec<(usize, usize)> = Vec::new();

    for i in 0..grid.len() {
        to_visit.push((i, 0));
        to_visit.push((i, grid[0].len() - 1));
    }

    for j in 0..grid[0].len() {
        to_visit.push((0, j));
        to_visit.push((grid.len() - 1, j));
    }

    while !to_visit.is_empty() {
        let (row, col) = to_visit.pop().unwrap();

        if row >= grid.len() || col >= grid[0].len() {
            continue;
        }

        if grid[row][col] == '.' {
            grid[row][col] = 'O';

            if row > 0 {
                to_visit.push((row - 1, col));
            }
            if col > 0 {
                to_visit.push((row, col - 1));
            }
            to_visit.push((row + 1, col));

            to_visit.push((row, col + 1));
        }
    }
}

fn count_dug(grid: &TrenchGrid) -> usize {
    grid.iter()
        .flat_map(|row| row.iter().filter(|c| **c != 'O'))
        .count()
}
