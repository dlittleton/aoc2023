use std::collections::{HashSet, VecDeque};

use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1, part2);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

type Garden = Grid<char>;

struct Path<'a> {
    seen: HashSet<Point>,
    goal_depth: usize,
    garden: &'a Garden,
    rows: usize,
    cols: usize,
}

impl<'a> Path<'a> {
    fn new(garden: &'a Garden, goal_depth: usize) -> Self {
        let seen = HashSet::new();

        let rows = garden.rows();
        let cols = garden.cols();

        Path {
            seen,
            goal_depth,
            garden,
            rows,
            cols,
        }
    }

    fn visit(&mut self, start: Point, start_depth: usize) -> usize {
        let mut to_visit: VecDeque<_> = VecDeque::new();
        to_visit.push_back((start, start_depth));

        let mut count = 0;

        while !to_visit.is_empty() {
            let (current, depth) = to_visit.pop_front().unwrap();

            if !self.is_passable(&current) || self.seen.contains(&current) {
                continue;
            }

            self.seen.insert(current);

            if depth % 2 == self.goal_depth % 2 {
                count += 1;
            }

            if depth == self.goal_depth {
                continue;
            }

            let d = depth + 1;
            to_visit.push_back((Point(current.0 - 1, current.1), d));
            to_visit.push_back((Point(current.0 + 1, current.1), d));
            to_visit.push_back((Point(current.0, current.1 - 1), d));
            to_visit.push_back((Point(current.0, current.1 + 1), d));
        }

        debug!("Estimate says {}", count);
        count
    }

    fn is_passable(&self, current: &Point) -> bool {
        return *self
            .garden
            .get(current.0 % self.rows, current.1 % self.cols)
            != '#';
    }
}

fn part1(lines: &[String]) -> String {
    let garden: Garden = lines.iter().map(|line| line.chars()).collect();
    let reachable = walk(&garden, 64);
    format!("{}", reachable)
}

fn part2(lines: &[String]) -> String {
    let garden: Garden = lines.iter().map(|line| line.chars()).collect();
    let start_pos = find_start(&garden);
    info!("Start is {:?}", start_pos);
    info!("Grid size is {} x {}", garden.rows(), garden.cols());

    // Had to look up a hint on this one. The total number of steps required
    // exactly lines up with a grid boundary. Because the grid is fairly open,
    // it works out that the number of available paths is quadratic.
    //
    // Solve the first few grid iterations to be able to calculate later values.
    // Goal step count of 26501365 steps is 202300 iterations
    let mut results: Vec<_> = Vec::new();

    for i in 0..7 {
        let steps = start_pos.0 + (garden.rows() * i);
        let paths = walk(&garden, steps);
        info!("{} : {}", i, paths);

        results.push(paths);
    }

    let diff = (results[2] - results[1]) - (results[1] - results[0]);
    info!("Diff is {}", diff);

    // Check
    let diff2 = (results[3] - results[2]) - (results[2] - results[1]);
    if diff != diff2 {
        panic!("Quadratic assumption did not hold");
    }

    let mut total = results[1];
    let mut last_diff = results[1] - results[0];
    for _ in 2..=202300 {
        last_diff += diff;
        total += last_diff;
    }

    format!("{}", total)
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

    // Offset start to avoid overflow.
    let offset = Point(
        start.0 + (garden.rows() * steps),
        start.1 + (garden.cols() * steps),
    );

    let mut path = Path::new(garden, steps);
    path.visit(offset, 0)
}
