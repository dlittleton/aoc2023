use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

use aoc2023::util::get_all_numbers;

lazy_static! {
    static ref RE_BROKEN: Regex = Regex::new(r"(#+)").unwrap();
}

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let total: usize = lines.iter().map(|line| count_variations(line)).sum();
    format!("{}", total)
}

fn count_variations(line: &String) -> usize {
    let (spec, values) = line.split_once(' ').unwrap();
    let runs: Vec<_> = get_all_numbers(values);
    let springs: Vec<_> = spec.chars().collect();

    let state = State::new(&springs, &runs);
    state.count_successors()
}

#[derive(Debug)]
struct State<'a> {
    current_run: usize,
    current_spring: char,
    remaining: &'a [char],
    runs: &'a [usize],
}

impl<'a> State<'a> {
    fn new(springs: &'a [char], runs: &'a [usize]) -> Self {
        Self {
            current_run: 0,
            current_spring: '.',
            remaining: springs,
            runs,
        }
    }

    fn is_valid(&self) -> bool {
        if self.current_spring == '#' {
            // An on-going run is only valid if it will be shorter than the next expected run.
            if self.runs.is_empty() {
                return false;
            } else {
                return self.current_run + 1 <= self.runs[0];
            }
        } else if self.current_run > 0 {
            // A run that is stopping must exactly match the next expected run.
            return self.current_run == self.runs[0];
        } else {
            // Empty run, still valid
            return true;
        }
    }

    fn child(&self, next_spring: char) -> Self {
        let mut child_run = self.current_run + 1;
        let mut child_remaining_runs = self.runs;

        if self.current_spring == '.' {
            child_run = 0;

            if self.current_run > 0 {
                child_remaining_runs = &self.runs[1..];
            }
        }

        Self {
            current_run: child_run,
            current_spring: next_spring,
            remaining: &self.remaining[1..],
            runs: child_remaining_runs,
        }
    }

    fn complete(&self) -> usize {
        let valid = (self.current_run == 0 && self.runs.is_empty())
            || (self.current_spring == '.'
                && self.current_run > 0
                && self.runs.len() == 1
                && self.current_run == self.runs[0])
            || (self.current_spring == '#'
                && self.runs.len() == 1
                && self.current_run + 1 == self.runs[0]);

        debug!("{:?}", self);
        debug!("Valid {}", valid);

        if valid {
            1
        } else {
            0
        }
    }

    fn count_successors(&self) -> usize {
        if !self.is_valid() {
            return 0;
        }

        match self.remaining {
            [] => self.complete(),
            ['?', _tail @ ..] => {
                self.child('#').count_successors() + self.child('.').count_successors()
            }
            [x, _tail @ ..] => self.child(*x).count_successors(),
        }
    }
}
