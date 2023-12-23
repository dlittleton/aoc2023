use std::collections::HashSet;

use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);

struct Maze {
    grid: Grid<char>,
}

impl Maze {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn left(&self, p: &Point) -> Option<Point> {
        if p.1 > 0 {
            return match self.grid.get(p.0, p.1) {
                '<' | '.' => Some(Point(p.0, p.1 - 1)),
                _ => None,
            };
        }
        None
    }

    fn up(&self, p: &Point) -> Option<Point> {
        if p.0 > 0 {
            return match self.grid.get(p.0, p.1) {
                '^' | '.' => Some(Point(p.0 - 1, p.1)),
                _ => None,
            };
        }
        None
    }

    fn right(&self, p: &Point) -> Option<Point> {
        if p.1 + 1 < self.grid.cols() {
            return match self.grid.get(p.0, p.1) {
                '>' | '.' => Some(Point(p.0, p.1 + 1)),
                _ => None,
            };
        }
        None
    }

    fn down(&self, p: &Point) -> Option<Point> {
        if p.0 + 1 < self.grid.rows() {
            return match self.grid.get(p.0, p.1) {
                'v' | '.' => Some(Point(p.0 + 1, p.1)),
                _ => None,
            };
        }
        None
    }
}

fn part1(lines: &[String]) -> String {
    let maze = Maze::new(lines.iter().map(|line| line.chars()).collect());

    let start = get_open_position(&maze, 0);
    let end = get_open_position(&maze, maze.grid.rows() - 1);

    info!("Start position is {:?}", start);
    info!("End position is {:?}", end);

    let longest = search(&maze, &start, &end);

    format!("{}", longest)
}

fn search(maze: &Maze, start: &Point, end: &Point) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();

    visit(maze, start, end, 0, &mut seen)
}

fn visit(
    maze: &Maze,
    current: &Point,
    end: &Point,
    steps: usize,
    seen: &mut HashSet<Point>,
) -> usize {
    debug!("Visiting {:?}", current);
    if seen.contains(current) || *maze.grid.get(current.0, current.1) == '#' {
        debug!("Dead end");
        return 0;
    }

    if current == end {
        info!("Found a goal of length {}", steps);
        return steps;
    }

    let paths = vec![
        maze.left(current),
        maze.right(current),
        maze.up(current),
        maze.down(current),
    ];
    debug!("Children {:?}", paths);

    seen.insert(*current);

    let best = paths
        .iter()
        .map(|point| {
            point
                .as_ref()
                .and_then(|p| Some(visit(maze, &p, end, steps + 1, seen)))
        })
        .map(|s| s.unwrap_or(0))
        .max()
        .unwrap();

    seen.remove(current);

    return best;
}

fn get_open_position(maze: &Maze, row: usize) -> Point {
    maze.grid
        .row_wise_iter()
        .nth(row)
        .unwrap()
        .enumerate()
        .find_map(|(c, v)| match v {
            '.' => Some(Point(row, c)),
            _ => None,
        })
        .unwrap()
}
