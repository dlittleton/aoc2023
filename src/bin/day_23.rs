use std::collections::HashSet;

use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1, part2);

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

fn part2(lines: &[String]) -> String {
    let mut maze = Maze::new(lines.iter().map(|line| line.chars()).collect());

    maze.grid.enumerate_mut().for_each(|(_, _, v)| match v {
        'v' | '<' | '>' | '^' => *v = '.',
        _ => (),
    });

    let start = get_open_position(&maze, 0);
    let end = get_open_position(&maze, maze.grid.rows() - 1);

    info!("Start position is {:?}", start);
    info!("End position is {:?}", end);

    let longest = search(&maze, &start, &end);

    format!("{}", longest)
}

fn search(maze: &Maze, start: &Point, end: &Point) -> usize {
    let mut seen: HashSet<Point> = HashSet::new();

    visit(maze, start, end, &mut seen)
}

enum CommandType {
    Enter,
    Visit,
    Exit,
}

fn visit(maze: &Maze, start: &Point, end: &Point, seen: &mut HashSet<Point>) -> usize {
    let mut to_visit: Vec<_> = Vec::new();
    to_visit.push((CommandType::Visit, *start, 0));

    let mut best = 0;
    let mut loops: usize = 0;

    while !to_visit.is_empty() {
        loops += 1;

        if loops % 100000000 == 0 {
            info!("Ran {} iterations.", loops);
        }

        let (cmd, current, steps) = to_visit.pop().unwrap();

        match cmd {
            CommandType::Enter => {
                seen.insert(current);
            }
            CommandType::Visit => {
                debug!("Visiting {:?}", current);
                if seen.contains(&current) || *maze.grid.get(current.0, current.1) == '#' {
                    debug!("Dead end");
                    continue;
                }

                if current == *end {
                    debug!("Found a goal of length {}", steps);
                    best = best.max(steps);
                    continue;
                }

                to_visit.push((CommandType::Exit, current, steps + 1));

                let paths = vec![
                    maze.left(&current),
                    maze.right(&current),
                    maze.up(&current),
                    maze.down(&current),
                ];
                debug!("Children {:?}", paths);

                to_visit.extend(paths.iter().filter_map(|p| {
                    p.and_then(|child| Some((CommandType::Visit, child, steps + 1)))
                }));

                to_visit.push((CommandType::Enter, current, steps + 1));
            }
            CommandType::Exit => {
                seen.remove(&current);
            }
        }
    }

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
