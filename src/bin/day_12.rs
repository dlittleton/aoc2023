use std::collections::HashMap;

use aoc2023::util::get_all_numbers;
use log::{debug, info};

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let total: usize = lines.iter().map(|line| count_variations(line)).sum();
    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let total: usize = lines.iter().map(|line| count_folded_variations(line)).sum();
    format!("{}", total)
}

fn count_variations(line: &String) -> usize {
    let (spec, values) = line.split_once(' ').unwrap();
    let runs: Vec<_> = get_all_numbers(values);
    let springs: Vec<_> = spec.chars().collect();

    let mut inspector = SpringInspector::new(springs, runs);
    inspector.count_permutations()
}

fn count_folded_variations(line: &String) -> usize {
    let (spec, values) = line.split_once(' ').unwrap();

    let unfolded_spec = std::iter::repeat(spec)
        .take(5)
        .collect::<Vec<&str>>()
        .join("?");

    let unfolded_values = std::iter::repeat(values)
        .take(5)
        .collect::<Vec<&str>>()
        .join(",");

    let runs: Vec<_> = get_all_numbers(&unfolded_values);
    let springs: Vec<_> = unfolded_spec.chars().collect();

    let mut inspector = SpringInspector::new(springs, runs);
    inspector.count_permutations()
}

struct SpringInspector {
    springs: Vec<char>,
    runs: Vec<usize>,
    cache: HashMap<State, usize>,
    cache_hits: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    pos: usize,
    run: usize,
    run_idx: usize,
}

impl SpringInspector {
    fn new(springs: Vec<char>, runs: Vec<usize>) -> Self {
        Self {
            springs,
            runs,
            cache: HashMap::new(),
            cache_hits: 0,
        }
    }

    fn count_permutations(&mut self) -> usize {
        let init = State {
            pos: 0,
            run: 0,
            run_idx: 0,
        };

        let total = self.successors(init);
        info!("Total was {}. Cache hits {}", total, self.cache_hits);
        return total;
    }

    fn successors(&mut self, state: State) -> usize {
        if let Some(cached) = self.cache.get(&state) {
            debug!("Using cached result for {:?} -- {}", state, cached);
            self.cache_hits += 1;
            return *cached;
        }

        let mut result = 0;

        if state.pos == self.springs.len() {
            // Reached the end. Only valid if either there's no active run and
            // no runs remaining or the active run matches the last remaining
            // run to check.
            let remaining = self.runs.len() - state.run_idx;

            if (state.run == 0 && remaining == 0)
                || (remaining == 1 && state.run == self.runs[state.run_idx])
            {
                return 1;
            } else {
                return 0;
            }
        }

        let current = self.springs[state.pos];

        if current == '#' || current == '?' {
            let child = State {
                pos: state.pos + 1,
                run: state.run + 1,
                run_idx: state.run_idx,
            };

            result += self.successors(child)
        }

        if current == '.' || current == '?' {
            let child = match (state.run, &self.runs[state.run_idx..]) {
                (0, _) => {
                    // No active run, descend with current remaining runs
                    Some(State {
                        pos: state.pos + 1,
                        run: 0,
                        run_idx: state.run_idx,
                    })
                }
                (_, []) => {
                    // Active run, but empty remaining matches, no possible solutions.
                    None
                }
                (x, [y, _tail @ ..]) if x != *y => {
                    // Run ended, length was wrong
                    None
                }
                (x, [y, _tail @ ..]) if x == *y => {
                    // Run ended, length matches, solution still possible
                    Some(State {
                        pos: state.pos + 1,
                        run: 0,
                        run_idx: state.run_idx + 1,
                    })
                }
                _ => panic!("Unhandled child state {:?}, {:?}", state, self.runs),
            };

            if child.is_some() {
                result += self.successors(child.unwrap());
            }
        }

        self.cache.insert(state, result);
        result
    }
}
