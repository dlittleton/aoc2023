use std::fmt::Display;

use aoc2023::util::get_all_numbers;
use core::ops::RangeInclusive;
use log::info;
use num::{bigint::ToBigInt, BigInt};

use vector3d::Vector3d;

aoc2023::solver!(part1, part2);

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

    fn to_stone(&self) -> Stone {
        let p = Vector3d::new(
            self.x.position.to_bigint().unwrap(),
            self.y.position.to_bigint().unwrap(),
            self.z.position.to_bigint().unwrap(),
        );

        let v = Vector3d::new(
            self.x.velocity.to_bigint().unwrap(),
            self.y.velocity.to_bigint().unwrap(),
            self.z.velocity.to_bigint().unwrap(),
        );

        return Stone { p, v };
    }
}

struct Stone {
    p: Vector3d<BigInt>,
    v: Vector3d<BigInt>,
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

/*
 * Needed a hint for this one.
 *
 * Implementation inspired by:
 * https://old.reddit.com/r/adventofcode/comments/18qexvu/2023_day_24_part_2_3d_vector_interpretation_and/
 */
fn part2(lines: &[String]) -> String {
    let stones: Vec<_> = lines
        .iter()
        .map(Path::parse)
        .map(|p| p.to_stone())
        .collect();

    let p0 = stones[0].p.clone();
    let v0 = stones[0].v.clone();

    // Shift all stones into a reference frame based on stone 0. Stone 0 is now
    // effectively stationary.
    let shifted: Vec<_> = stones
        .iter()
        .map(|s| Stone {
            p: s.p.clone() - p0.clone(),
            v: s.v.clone() - v0.clone(),
        })
        .collect();

    // Find normal line for stone 1.
    let h1 = &shifted[1];
    let h1_p0 = &h1.p;
    let h1_p1 = h1.p.clone() + h1.v.clone();
    let n = h1_p0.clone().cross(h1_p1);

    // Find intersection time and position for stone 2
    let h2 = &shifted[2];
    let t_h2 = (shifted[0].p.clone() - h2.p.clone()).dot(n.clone()) / h2.v.clone().dot(n.clone());
    let p_h2 = h2.p.clone() + (h2.v.clone() * t_h2.clone());

    // Find intersection time and position for stone 3
    let h3 = &shifted[3];
    let t_h3 = (shifted[0].p.clone() - h3.p.clone()).dot(n.clone()) / h3.v.clone().dot(n);
    let p_h3 = h3.p.clone() + (h3.v.clone() * t_h3.clone());

    // Calculate velocity and position of rock in the shifted reference frame.
    let velo = (p_h3 - p_h2.clone()) / (t_h3.clone() - t_h2.clone());
    let position = p_h2.clone() - (velo.clone() * t_h2);
    info!("Shifted velo {}, Shifted position {}", velo, position);

    // Unshift
    let real_position = position + stones[0].p.clone();
    info!("Real position is {}", real_position);

    let total = real_position.x + real_position.y + real_position.z;

    format!("{}", total)
}
