use std::collections::HashSet;

use aoc2023::collections::grid::Grid;

aoc2023::solver!(part1);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
struct State(Point, usize);

type Garden = Grid<char>;

struct Path<'a> {
    seen: HashSet<State>,
    goals: HashSet<Point>,
    goal_depth: usize,
    garden: &'a Garden,
}

impl<'a> Path<'a> {
    fn new(garden: &'a Garden, goal_depth: usize) -> Self {
        let seen = HashSet::new();
        let goals = HashSet::new();

        Path {
            seen,
            goals,
            goal_depth,
            garden,
        }
    }

    fn visit(&mut self, current: Point, depth: usize) {
        let state = State(current, depth);

        if !self.is_passable(&current) || self.seen.contains(&state) {
            return;
        }

        self.seen.insert(state);

        if depth == self.goal_depth {
            self.goals.insert(current);
            return;
        }

        let d = depth + 1;
        // Up
        if current.0 > 0 {
            self.visit(Point(current.0 - 1, current.1), d);
        }

        // Left
        if current.1 > 0 {
            self.visit(Point(current.0, current.1 - 1), d);
        }

        // Down
        self.visit(Point(current.0 + 1, current.1), d);

        // Right
        self.visit(Point(current.0, current.1 + 1), d);
    }

    fn is_passable(&self, current: &Point) -> bool {
        return current.0 < self.garden.rows()
            && current.1 < self.garden.cols()
            && *self.garden.get(current.0, current.1) != '#';
    }
}

fn part1(lines: &[String]) -> String {
    let garden: Garden = lines.iter().map(|line| line.chars()).collect();
    let reachable = walk(&garden, 64);
    format!("{}", reachable)
}

fn find_start(garden: &Garden) -> Point {
    garden
        .enumerate()
        .find_map(|(r, c, v)| match v {
            'S' => Some(Point(r, c)),
            _ => None,
        })
        .unwrap()
}

fn walk(garden: &Garden, steps: usize) -> usize {
    let start = find_start(garden);

    let mut path = Path::new(garden, steps);
    path.visit(start, 0);

    return path.goals.len();
}
