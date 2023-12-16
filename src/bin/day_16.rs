use std::collections::HashSet;

use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1, part2);

#[derive(Debug)]
struct Tile {
    contents: char,
    visit_dirs: HashSet<Direction>,
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn part1(lines: &[String]) -> String {
    let mut grid = parse_tiles(lines);

    route_beam(Position(0, 0), Direction::Right, &mut grid);

    let energized = grid
        .enumerate()
        .filter(|(_, _, tile)| !tile.visit_dirs.is_empty())
        .count();
    format!("{}", energized)
}

fn part2(lines: &[String]) -> String {
    let mut grid = parse_tiles(lines);

    let rows = grid.rows();
    let cols = grid.cols();

    // Possible starting points.
    let points = (0..cols)
        .map(|c| (Position(0, c), Direction::Down))
        .chain((0..cols).map(|c| (Position(rows - 1, c), Direction::Up)))
        .chain((0..rows).map(|r| (Position(r, 0), Direction::Right)))
        .chain((0..rows).map(|r| (Position(r, cols - 1), Direction::Left)));

    let best = points
        .map(|start| {
            // Clear previous values
            grid.enumerate_mut()
                .for_each(|(_, _, tile)| tile.visit_dirs.clear());

            // Route beam
            route_beam(start.0, start.1, &mut grid);

            // Calculate score
            let score = grid
                .enumerate()
                .filter(|(_, _, tile)| !tile.visit_dirs.is_empty())
                .count();

            info!(
                "Starting at {:?} going {:?} score is {}",
                start.0, start.1, score
            );

            return score;
        })
        .max()
        .unwrap();

    format!("{}", best)
}

fn route_beam(pos: Position, dir: Direction, grid: &mut Grid<Tile>) {
    if grid.get(pos.0, pos.1).visit_dirs.contains(&dir) {
        debug!("Skipping position {:?} in direction {:?}", pos, dir);
        return;
    }

    grid.get_mut(pos.0, pos.1).visit_dirs.insert(dir);

    let contents = grid.get(pos.0, pos.1).contents;

    let next_dirs = match (dir, contents) {
        // On empty space, continue in current direction
        (d, '.') => vec![d],

        // Inactive Splitters
        (Direction::Up, '|') => vec![Direction::Up],
        (Direction::Down, '|') => vec![Direction::Down],
        (Direction::Left, '-') => vec![Direction::Left],
        (Direction::Right, '-') => vec![Direction::Right],

        // Active Splitters
        (Direction::Up, '-') | (Direction::Down, '-') => vec![Direction::Left, Direction::Right],
        (Direction::Left, '|') | (Direction::Right, '|') => vec![Direction::Up, Direction::Down],

        // Redirects
        (Direction::Up, '/') => vec![Direction::Right],
        (Direction::Up, '\\') => vec![Direction::Left],

        (Direction::Down, '/') => vec![Direction::Left],
        (Direction::Down, '\\') => vec![Direction::Right],

        (Direction::Left, '/') => vec![Direction::Down],
        (Direction::Left, '\\') => vec![Direction::Up],

        (Direction::Right, '/') => vec![Direction::Up],
        (Direction::Right, '\\') => vec![Direction::Down],

        (d, c) => {
            panic!("Unmatched direction {:?} {}", d, c)
        }
    };

    for d in next_dirs.iter() {
        match (d, &pos) {
            (Direction::Up, p) if p.0 > 0 => route_beam(Position(p.0 - 1, p.1), *d, grid),
            (Direction::Down, p) if p.0 + 1 < grid.rows() => {
                route_beam(Position(p.0 + 1, p.1), *d, grid)
            }
            (Direction::Left, p) if p.1 > 0 => route_beam(Position(p.0, p.1 - 1), *d, grid),
            (Direction::Right, p) if p.1 + 1 < grid.cols() => {
                route_beam(Position(p.0, p.1 + 1), *d, grid)
            }
            (_, _) => (),
        }
    }
}

fn parse_tiles(lines: &[String]) -> Grid<Tile> {
    lines
        .iter()
        .map(|line| {
            line.chars().map(|c| Tile {
                contents: c,
                visit_dirs: HashSet::new(),
            })
        })
        .collect()
}
