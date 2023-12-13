use aoc2023::collections::grid::Grid;

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let grids = parse_grids(lines);

    let mut vertical = 0;
    let mut horizontal = 0;
    for g in grids {
        let mut matches = 0;
        if let Some(v) = find_vertical_reflection(&g) {
            vertical += v;
            matches += 1;
        }

        if let Some(h) = find_horizontal_reflection(&g) {
            horizontal += h;
            matches += 1;
        }

        println!("Matches {}", matches);
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

fn find_vertical_reflection(grid: &Grid<char>) -> Option<usize> {
    let cols: Vec<Vec<_>> = grid
        .col_wise_iter()
        .map(|c| c.map(|v| *v).collect())
        .collect();

    find_symmetry(&cols)
}

fn find_horizontal_reflection(grid: &Grid<char>) -> Option<usize> {
    let rows: Vec<Vec<_>> = grid
        .row_wise_iter()
        .map(|r| r.map(|v| *v).collect())
        .collect();

    find_symmetry(&rows)
}

fn find_symmetry(data: &Vec<Vec<char>>) -> Option<usize> {
    // Need at least one entry on each side of the reflection
    for axis_point in 1..data.len() {
        let mut a = axis_point - 1;
        let mut b = axis_point;

        loop {
            if data[a] != data[b] {
                break;
            }

            if a == 0 || b == data.len() - 1 {
                return Some(axis_point);
            }

            a -= 1;
            b += 1;
        }
    }

    None
}
