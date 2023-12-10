use std::collections::HashMap;

use log::debug;

aoc2023::solver!(part1);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize);
type Grid = Vec<Vec<char>>;

fn part1(lines: &[String]) -> String {
    let grid = parse_grid(lines);
    debug!("{:?}", grid);

    let furthest = get_furthest_position(&grid);

    format!("{}", furthest)
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

fn get_start_neighboars(start: &Point, grid: &Grid) -> Vec<Point> {
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

fn get_furthest_position(grid: &Grid) -> usize {
    let start = get_start_position(&grid);

    let mut distances: HashMap<Point, usize> = HashMap::new();

    let neighbors = get_start_neighboars(&start, grid);
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

    *distances.values().max().unwrap()
}
