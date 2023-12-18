use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let grid = parse_grid(lines);

    let loss = search(&grid, 0, 3);

    format!("{}", loss)
}

fn part2(lines: &[String]) -> String {
    let grid = parse_grid(lines);

    let loss = search(&grid, 4, 10);

    format!("{}", loss)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
struct Path {
    heat_loss: usize,
    distance_remaining: usize,
    pos: Position,
    direction: Direction,
    steps_without_turn: usize,
    depth: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct PathState {
    pos: Position,
    direction: Direction,
    steps_without_turn: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hd = self.heat_loss + self.distance_remaining;
        let other_hd = other.heat_loss + other.distance_remaining;
        hd.cmp(&other_hd)
    }
}

impl Path {
    fn children(&self, grid: &Grid<usize>, min_steps: usize, max_steps: usize) -> Vec<Self> {
        let goal = Position(grid.rows() - 1, grid.cols() - 1);
        let total_heat_loss = grid.get(self.pos.0, self.pos.1) + self.heat_loss;

        // Turns
        let mut directions = match self.direction {
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
        };

        if self.steps_without_turn < min_steps {
            directions.clear(); // Can't turn until min steps
        }

        if self.steps_without_turn < max_steps {
            directions.push(self.direction);
        }

        directions
            .iter()
            .filter_map(|d| {
                let new_pos = match (d, &self.pos) {
                    (Direction::Up, p) if p.0 > 0 => Some(Position(p.0 - 1, p.1)),
                    (Direction::Down, p) if p.0 + 1 < grid.rows() => Some(Position(p.0 + 1, p.1)),
                    (Direction::Left, p) if p.1 > 0 => Some(Position(p.0, p.1 - 1)),
                    (Direction::Right, p) if p.1 + 1 < grid.cols() => Some(Position(p.0, p.1 + 1)),
                    _ => None,
                };

                let steps = if *d == self.direction {
                    self.steps_without_turn + 1
                } else {
                    1
                };

                new_pos.and_then(|pos| {
                    Some(Path {
                        heat_loss: total_heat_loss,
                        distance_remaining: (goal.0 - pos.0) + (goal.1 - pos.1),
                        pos: pos,
                        direction: *d,
                        steps_without_turn: steps,
                        depth: self.depth + 1,
                    })
                })
            })
            .collect()
    }
}

fn search(grid: &Grid<usize>, min_steps: usize, max_steps: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen: HashMap<PathState, usize> = HashMap::new();
    let goal = Position(grid.rows() - 1, grid.cols() - 1);

    // Starting paths
    heap.push(Reverse(Path {
        heat_loss: 0,
        distance_remaining: goal.0 + goal.1 - 1,
        pos: Position(0, 1),
        direction: Direction::Right,
        steps_without_turn: 1,
        depth: 0,
    }));

    heap.push(Reverse(Path {
        heat_loss: 0,
        distance_remaining: goal.0 - 1 + goal.1,
        pos: Position(1, 0),
        direction: Direction::Down,
        steps_without_turn: 1,
        depth: 0,
    }));

    loop {
        let current = heap.pop().unwrap().0;

        let state = PathState {
            pos: current.pos,
            direction: current.direction,
            steps_without_turn: current.steps_without_turn,
        };

        if let Some(cached) = seen.get(&state) {
            if *cached <= current.heat_loss {
                debug!("Skipping node!");
                continue;
            }
        }
        seen.insert(state, current.heat_loss);

        if current.pos == goal {
            if current.steps_without_turn < min_steps {
                info!("Ignoring solution because it has not traveled {} steps before reaching the end.", min_steps);
                continue;
            }

            return current.heat_loss + grid.get(goal.0, goal.1);
        }

        let children = current.children(grid, min_steps, max_steps);
        for child in children.into_iter() {
            heap.push(Reverse(child));
        }
    }
}

fn parse_grid(lines: &[String]) -> Grid<usize> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<usize>().unwrap())
        })
        .collect()
}
