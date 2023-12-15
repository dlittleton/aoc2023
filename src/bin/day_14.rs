use std::collections::HashMap;

use log::info;

aoc2023::solver!(part1, part2);

type RockGrid = Vec<Vec<char>>;

fn part1(lines: &[String]) -> String {
    let mut grid: RockGrid = lines.iter().map(|line| line.chars().collect()).collect();

    shift_up(&mut grid);
    let total = calculate_load(&grid);
    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let mut grid: RockGrid = lines.iter().map(|line| line.chars().collect()).collect();

    //let count = 1000000000;
    let count = 5000;
    cycle(&mut grid, count);

    let total = calculate_load(&grid);
    format!("{}", total)
}

fn cycle(grid: &mut RockGrid, count: usize) {
    let mut value_cache: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 1..count {
        shift_up(grid);
        shift_left(grid);
        shift_down(grid);
        shift_right(grid);

        let total = calculate_load(grid);
        if let Some(cached) = value_cache.get_mut(&total) {
            cached.push(i);
            if cached.len() > 25 {
                for (k, v) in value_cache.iter() {
                    if v.len() >= 10 {
                        println!("{}\t{:?}", k, v);
                    }
                }
                break;
            }
        } else {
            value_cache.insert(total, vec![i]);
        }
    }
}

fn shift_up(grid: &mut RockGrid) {
    for c in 0..grid[0].len() {
        let mut next_idx = 0;
        for r in 0..grid.len() {
            if grid[r][c] == 'O' {
                grid[r][c] = '.';
                grid[next_idx][c] = 'O';
                next_idx += 1;
            } else if grid[r][c] == '#' {
                next_idx = r + 1
            }
        }
    }
}

fn shift_left(grid: &mut RockGrid) {
    for r in 0..grid.len() {
        let mut next_idx = 0;
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                grid[r][c] = '.';
                grid[r][next_idx] = 'O';
                next_idx += 1;
            } else if grid[r][c] == '#' {
                next_idx = c + 1
            }
        }
    }
}

fn shift_down(grid: &mut RockGrid) {
    for c in 0..grid[0].len() {
        let mut next_idx = grid.len() - 1;
        for r in (0..grid.len()).rev() {
            if grid[r][c] == 'O' {
                grid[r][c] = '.';
                grid[next_idx][c] = 'O';
                if next_idx > 0 {
                    next_idx -= 1;
                }
            } else if grid[r][c] == '#' && r > 0 {
                next_idx = r - 1
            }
        }
    }
}

fn shift_right(grid: &mut RockGrid) {
    for r in 0..grid.len() {
        let mut next_idx = grid[0].len() - 1;
        for c in (0..grid[0].len()).rev() {
            if grid[r][c] == 'O' {
                grid[r][c] = '.';
                grid[r][next_idx] = 'O';
                if next_idx > 0 {
                    next_idx -= 1;
                }
            } else if grid[r][c] == '#' && c > 0 {
                next_idx = c - 1
            }
        }
    }
}

fn calculate_load(grid: &RockGrid) -> usize {
    let mut total = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                total += grid.len() - r;
            }
        }
    }

    total
}

fn dump_grid(grid: &RockGrid) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            print!("{}", grid[r][c]);
        }
        println!("");
    }
}
