use std::collections::HashSet;

use aoc2023::util::get_all_numbers;
use log::{debug, info};

aoc2023::solver!(part1, part2);

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32, i32);

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn parse(line: &String) -> Self {
        let (a, b) = line.split_once('~').unwrap();

        let c1 = get_all_numbers(a);
        let start = Point(c1[0], c1[1], c1[2]);

        let c2 = get_all_numbers(b);
        let end = Point(c2[0], c2[1], c2[2]);

        Self { start, end }
    }

    fn is_on(&self, other: &Self) -> bool {
        debug!("Check {:?} is on {:?}", self, other);
        // Can't be on the other brick if the z distance is greater than 1
        if self.start.2 != other.end.2 + 1 {
            return false;
        }

        let x_clear = self.end.0 < other.start.0 || self.start.0 > other.end.0;
        let y_clear = self.end.1 < other.start.1 || self.start.1 > other.end.1;

        if x_clear || y_clear {
            return false;
        } else {
            return true;
        }
    }

    fn fall(&mut self) {
        self.start = Point(self.start.0, self.start.1, self.start.2 - 1);
        self.end = Point(self.end.0, self.end.1, self.end.2 - 1);
    }
}

fn part1(lines: &[String]) -> String {
    let mut bricks: Vec<_> = lines.iter().map(Brick::parse).collect();
    settle(&mut bricks);

    let removable = find_removable(&bricks);
    format!("{}", removable)
}

fn part2(lines: &[String]) -> String {
    let mut bricks: Vec<_> = lines.iter().map(Brick::parse).collect();
    settle(&mut bricks);

    let will_fall = find_chain(&bricks);
    format!("{}", will_fall)
}

fn settle(bricks: &mut [Brick]) -> usize {
    let mut fell: Vec<_> = bricks.iter().map(|_| false).collect();

    let mut to_settle: HashSet<_> = bricks.iter().enumerate().map(|(i, _)| i).collect();

    while !to_settle.is_empty() {
        let remaining: Vec<_> = to_settle.iter().map(|i| *i).collect();
        for i in remaining {
            debug!("Looking at brick {}", i);
            // Ground
            if bricks[i].start.2 == 1 {
                debug!("Brick at {} is touching the ground.", i);
                to_settle.remove(&i);
                continue;
            }

            if let Some(support) = bricks
                .iter()
                .enumerate()
                .find(|(j, b)| i != *j && bricks[i].is_on(b))
            {
                debug!("Found overlap with {} on {}", i, support.0);
                if !to_settle.contains(&support.0) {
                    debug!("Brick {} is resting on brick {}", i, support.0);
                    to_settle.remove(&i);
                }
            } else {
                debug!("Moving brick {} down.", i);
                fell[i] = true;
                bricks[i].fall();
            }
        }
    }

    let total = fell.iter().filter(|b| **b).count();
    info!("A total of {} bricks fell.", total);
    total
}

fn find_removable(bricks: &[Brick]) -> usize {
    let mut removable: HashSet<_> = bricks.iter().enumerate().map(|(i, _)| i).collect();

    for i in 0..bricks.len() {
        let supports: Vec<_> = bricks
            .iter()
            .enumerate()
            .filter(|(j, b)| i != *j && bricks[i].is_on(b))
            .collect();

        if supports.len() == 1 {
            info!(
                "Brick {} is not removable because it supports brick {}",
                supports[0].0, i
            );
            removable.remove(&supports[0].0);
        }
    }

    return removable.len();
}

fn find_chain(bricks: &Vec<Brick>) -> usize {
    let mut total = 0;

    for i in 0..bricks.len() {
        info!("Removing brick {}", i);
        let mut copied = bricks.to_owned();
        copied.remove(i);

        total += settle(&mut copied);
    }

    total
}
