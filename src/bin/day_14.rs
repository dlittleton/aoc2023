use aoc2023::collections::grid::Grid;

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let grid: Grid<_> = lines.iter().map(|line| line.chars()).collect();

    let total = calculate_load(&grid);
    format!("{}", total)
}

fn calculate_load(grid: &Grid<char>) -> usize {
    let mut total = 0;

    for col in grid.col_wise_iter() {
        let mut next_idx = 0;
        for (i, c) in col.enumerate() {
            if *c == 'O' {
                total += grid.rows() - next_idx;
                next_idx += 1;
            } else if *c == '#' {
                next_idx = i + 1;
            }
        }
    }

    total
}
