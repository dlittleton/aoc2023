use aoc2023::util::get_all_numbers;
use log::info;

aoc2023::solver!(part1);

fn find_next_number(values: &[i32]) -> i32 {
    let diff: Vec<_> = values[..]
        .iter()
        .zip(values[1..].iter())
        .map(|(a, b)| b - a)
        .collect();

    info!("{:?}", diff);

    if diff.iter().all(|v| *v == 0) {
        return *values.last().unwrap();
    } else {
        return values.last().unwrap() + find_next_number(&diff);
    }
}

fn part1(lines: &[String]) -> String {
    let total: i32 = lines
        .iter()
        .map(|line| {
            let values = get_all_numbers(line);
            find_next_number(&values)
        })
        .sum();

    format!("{}", total)
}
