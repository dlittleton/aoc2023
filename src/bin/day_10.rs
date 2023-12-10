use std::collections::HashMap;

use log::{debug, info, log_enabled};

aoc2023::solver!(part1, part2);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize);
type Grid = Vec<Vec<char>>;
type PointMap = HashMap<Point, usize>;

fn part1(lines: &[String]) -> String {
    let grid = parse_grid(lines);
    debug!("{:?}", grid);

    let furthest = *get_path(&grid).values().max().unwrap();

    format!("{}", furthest)
}

fn part2(lines: &[String]) -> String {
    let mut grid = parse_grid(lines);
    debug!("{:?}", grid);

    let path = get_path(&grid);
    clear_grid(&mut grid, &path);
    dump_grid(&grid);

    let mut expanded = expand_grid(&grid);
    dump_grid(&expanded);

    fill(&mut expanded);
    dump_grid(&expanded);

    let count = expanded
        .iter()
        .enumerate()
        .map(|(i, v)| v.iter().enumerate().map(move |(j, c)| (i, j, *c)))
        .flatten()
        .filter(|(i, j, c)| i % 2 == 0 && j % 2 == 0 && *c == '.')
        .count();

    format!("{}", count)
}

fn clear_grid(grid: &mut Grid, path: &PointMap) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let position = Point(i, j);
            if !path.contains_key(&position) {
                grid[i][j] = '.'
            }
        }
    }
}

fn dump_grid(grid: &Grid) {
    if log_enabled!(log::Level::Info) {
        for i in 0..grid.len() {
            let output: String = grid[i].iter().collect();
            info!("{}", output);
        }
    }
}

fn parse_grid(lines: &[String]) -> Grid {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn get_start_position(grid: &Grid) -> Point {
    for (i, v) in grid.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == 'S' {
                return Point(i, j);
            }
        }
    }

    panic!("Start position not found!");
}

fn get_start_neighbors(start: &Point, grid: &Grid) -> Vec<Point> {
    let mut neighbors: Vec<_> = Vec::new();

    // Up
    if start.0 > 0 {
        let row = start.0 - 1;
        let col = start.1;
        let c = grid[row][col];
        if c == '|' || c == '7' || c == 'F' {
            neighbors.push(Point(row, col))
        }
    }

    // Down
    if start.0 + 1 < grid.len() {
        let row = start.0 + 1;
        let col = start.1;
        let c = grid[row][col];
        if c == '|' || c == 'L' || c == 'J' {
            neighbors.push(Point(row, col))
        }
    }

    // Left
    if start.1 > 0 {
        let row = start.0;
        let col = start.1 - 1;
        let c = grid[row][col];
        if c == '-' || c == 'F' || c == 'L' {
            neighbors.push(Point(row, col))
        }
    }

    // Right
    if start.1 + 1 < grid[0].len() {
        let row = start.0;
        let col = start.1 + 1;
        let c = grid[row][col];
        if c == '-' || c == 'J' || c == '7' {
            neighbors.push(Point(row, col))
        }
    }

    neighbors
}

fn get_next_position(previous: &Point, current: &Point, grid: &Grid) -> Point {
    let neighbors = match grid[current.0][current.1] {
        '|' => [
            Point(current.0 - 1, current.1),
            Point(current.0 + 1, current.1),
        ],
        '-' => [
            Point(current.0, current.1 - 1),
            Point(current.0, current.1 + 1),
        ],
        'L' => [
            Point(current.0 - 1, current.1),
            Point(current.0, current.1 + 1),
        ],
        'J' => [
            Point(current.0 - 1, current.1),
            Point(current.0, current.1 - 1),
        ],
        '7' => [
            Point(current.0 + 1, current.1),
            Point(current.0, current.1 - 1),
        ],
        'F' => [
            Point(current.0 + 1, current.1),
            Point(current.0, current.1 + 1),
        ],
        x => panic!("Unexpected grid value {}", x),
    };

    neighbors
        .into_iter()
        .filter(|p| p != previous)
        .next()
        .unwrap()
}

fn get_path(grid: &Grid) -> HashMap<Point, usize> {
    let start = get_start_position(&grid);

    let mut distances: HashMap<Point, usize> = HashMap::new();
    distances.insert(start, 0);

    let neighbors = get_start_neighbors(&start, grid);
    if neighbors.len() != 2 {
        panic!("Unexpected neighbor count {:?}", neighbors);
    }

    let mut next1 = neighbors[0];
    let mut next2 = neighbors[1];

    let mut prev1 = start;
    let mut prev2 = start;

    let mut steps = 1;
    while !distances.contains_key(&next1) && !distances.contains_key(&next2) {
        distances.insert(next1, steps);
        distances.insert(next2, steps);

        let temp = next1;
        next1 = get_next_position(&prev1, &next1, grid);
        prev1 = temp;

        let temp = next2;
        next2 = get_next_position(&prev2, &next2, grid);
        prev2 = temp;

        steps += 1;
    }

    distances
}

/// Expand that paths so that spaces between pipes correspond to coordinates
fn expand_grid(grid: &Grid) -> Grid {
    let mut result: Grid = Grid::new();
    for i in 0..grid.len() {
        let mut row: Vec<_> = Vec::new();
        for j in 0..grid[0].len() {
            let current = grid[i][j];
            if j > 0 {
                let prev = grid[i][j - 1];

                if "SFL-".contains(prev) && "SJ7-".contains(current) {
                    row.push('-')
                } else {
                    row.push('.')
                }
            }
            row.push(current)
        }

        if i > 0 {
            let prev = result.last().unwrap();

            let spacer: Vec<_> = prev
                .iter()
                .zip(row.iter())
                .map(|(p, c)| {
                    if "SF7|".contains(*p) && "SLJ|".contains(*c) {
                        '|'
                    } else {
                        '.'
                    }
                })
                .collect();

            result.push(spacer)
        }

        result.push(row);
    }

    result
}

fn fill(grid: &mut Grid) {
    let mut to_visit: Vec<Point> = Vec::new();

    for i in 0..grid.len() {
        to_visit.push(Point(i, 0));
        to_visit.push(Point(i, grid[0].len() - 1));
    }

    for j in 0..grid[0].len() {
        to_visit.push(Point(0, j));
        to_visit.push(Point(grid.len() - 1, j));
    }

    while !to_visit.is_empty() {
        let pos = to_visit.pop().unwrap();

        if pos.0 >= grid.len() || pos.1 >= grid[0].len() {
            continue;
        }

        if grid[pos.0][pos.1] == '.' {
            grid[pos.0][pos.1] = 'O';

            if pos.0 > 0 {
                to_visit.push(Point(pos.0 - 1, pos.1));
            }
            if pos.1 > 0 {
                to_visit.push(Point(pos.0, pos.1 - 1));
            }
            to_visit.push(Point(pos.0 + 1, pos.1));

            to_visit.push(Point(pos.0, pos.1 + 1));
        }
    }
}
