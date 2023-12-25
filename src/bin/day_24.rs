use std::fmt::Display;

use aoc2023::util::get_all_numbers;
use core::ops::RangeInclusive;
use log::{debug, info};

aoc2023::solver!(part1);

#[derive(Debug)]
struct Line {
    velocity: i64,
    position: i64,
    v: f64,
    p: f64,
}

impl Line {
    fn new(velocity: i64, position: i64) -> Self {
        Self {
            velocity,
            position,
            v: velocity as f64,
            p: position as f64,
        }
    }

    // Find position based on time
    fn apply(&self, t: f64) -> f64 {
        (self.v * t) + self.p
    }

    // Find time based on position
    fn solve(&self, pos: f64) -> f64 {
        (pos - self.p) / self.v
    }
}

#[derive(Debug)]
struct Path {
    x: Line,
    y: Line,
    z: Line,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.x.position,
            self.y.position,
            self.z.position,
            self.x.velocity,
            self.y.velocity,
            self.z.velocity
        )
    }
}

impl Path {
    fn parse(line: &String) -> Self {
        let (pos, change) = line.split_once('@').unwrap();

        let p = get_all_numbers::<i64>(pos);
        let s = get_all_numbers::<i64>(change);

        let x = Line::new(s[0], p[0]);
        let y = Line::new(s[1], p[1]);
        let z = Line::new(s[2], p[2]);

        Self { x, y, z }
    }

    fn cross_2d(&self, other: &Self, valid: &RangeInclusive<f64>) -> bool {
        info!("Hailstone A: {}", self);
        info!("Hailstone B: {}", other);
        if let Some(xpos) = find_intersection(&self.x, &self.y, &other.x, &other.y) {
            let t1 = self.x.solve(xpos);
            let t2 = other.x.solve(xpos);

            let y1 = self.y.apply(t1);

            if t1 < 0_f64 || t2 < 0_f64 {
                info!("Intersection occurred in the past.");
                return false;
            }

            if !valid.contains(&xpos) || !valid.contains(&y1) {
                info!("Intersection out of range {}, {}", xpos, y1);
                return false;
            }

            info!("FOUND intersection {}, {}", xpos, y1);
            return true;
        }

        info!("No intersection.");
        return false;
    }
}

fn find_intersection(a1: &Line, b1: &Line, a2: &Line, b2: &Line) -> Option<f64> {
    let left_const = (a1.p * a2.v * b1.v * -1.0) + (b1.p * a1.v * a2.v);
    let right_const = (a2.p * a1.v * b2.v * -1.0) + (b2.p * a1.v * a2.v);

    let n_x = (b1.v * a2.v) - (b2.v * a1.v);
    let c = right_const - left_const;

    let cross = c / n_x;
    if cross.is_finite() {
        return Some(cross);
    } else {
        return None;
    }
}

fn part1(lines: &[String]) -> String {
    let paths: Vec<_> = lines.iter().map(Path::parse).collect();

    let min_value = 200000000000000_f64;
    let max_value = 400000000000000_f64;

    //let min_value = 7_f64;
    //let max_value = 27_f64;

    let valid_range = min_value..=max_value;
    let mut count = 0;

    for i in 0..paths.len() - 1 {
        for j in i + 1..paths.len() {
            if paths[i].cross_2d(&paths[j], &valid_range) {
                count += 1;
            }
        }
    }

    format!("{}", count)
}
