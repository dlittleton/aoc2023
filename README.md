# aoc2023
Advent of Code 2023

https://adventofcode.com/2023

An attempt to learn some rust while solving AOC puzzles.

## Organization

Solutions are implemented as separate bin targets under `src/bin`.

Common code can be found in `src/lib`.

A macro is used to simplify declaring the main method, parsing input, and switching between part one and part two.

```
aoc2023::solver!(solve_part1, solve_part2);

fn solve_part1(lines: &[String]) -> String {
    // Solution here
}

fn solve_part2(lines: &[String]) -> String {
    // Solution here
}
```

## Running

- Use `-2` to run part 2.
- Use `RUST_LOG` env var to set log level

```
> RUST_LOG=info cargo run --bin day_n -- <input_file> [-2]
```