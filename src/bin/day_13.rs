use aoc2023::collections::grid::Grid;
use log::{debug, info};

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let grids = parse_grids(lines);

    let mut vertical = 0;
    let mut horizontal = 0;
    for g in grids {
        let mut matches = 0;
        if let Some(v) = find_vertical_reflection(&g, 0) {
            vertical += v;
            matches += 1;
        }

        if let Some(h) = find_horizontal_reflection(&g, 0) {
            horizontal += h;
            matches += 1;
        }

        info!("Matches {}", matches);
    }

    let total = vertical + (100 * horizontal);
    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let grids = parse_grids(lines);

    let mut vertical = 0;
    let mut horizontal = 0;
    for g in grids {
        let mut matches = 0;
        if let Some(v) = find_vertical_reflection(&g, 1) {
            vertical += v;
            matches += 1;
        }

        if let Some(h) = find_horizontal_reflection(&g, 1) {
            horizontal += h;
            matches += 1;
        }

        info!("Matches {}", matches);
    }

    let total = vertical + (100 * horizontal);
    format!("{}", total)
}

fn parse_grids(lines: &[String]) -> Vec<Grid<char>> {
    let mut grids: Vec<_> = Vec::new();

    let mut peekable_lines = lines.iter().peekable();
    while peekable_lines.peek().is_some() {
        let grid: Grid<_> = peekable_lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars())
            .collect();

        grids.push(grid);
    }

    grids
}

fn find_vertical_reflection(grid: &Grid<char>, target_diff: usize) -> Option<usize> {
    let cols: Vec<Vec<_>> = grid
        .col_wise_iter()
        .map(|c| c.map(|v| *v).collect())
        .collect();

    debug!("Checking for vertical symmetry");
    find_symmetry(&cols, target_diff)
}

fn find_horizontal_reflection(grid: &Grid<char>, target_diff: usize) -> Option<usize> {
    let rows: Vec<Vec<_>> = grid
        .row_wise_iter()
        .map(|r| r.map(|v| *v).collect())
        .collect();

    debug!("Checking for horizontal symmetry");
    find_symmetry(&rows, target_diff)
}

fn find_symmetry(data: &Vec<Vec<char>>, target_diff: usize) -> Option<usize> {
    // Need at least one entry on each side of the reflection
    for axis_point in 1..data.len() {
        let mut total_diff = 0;
        let mut a = axis_point - 1;
        let mut b = axis_point;

        loop {
            total_diff += data[a]
                .iter()
                .zip(data[b].iter())
                .filter(|(va, vb)| **va != **vb)
                .count();

            debug!("Axis {}, Total Diff is {}", axis_point, total_diff);

            if total_diff > target_diff {
                break;
            }

            if a == 0 || b == data.len() - 1 {
                if total_diff == target_diff {
                    return Some(axis_point);
                } else {
                    break;
                }
            }

            a -= 1;
            b += 1;
        }
    }

    None
}
