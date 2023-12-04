use regex::Regex;
use std::collections::HashSet;

aoc2023::solver!(part1);

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Point(i32, i32);

fn part1(lines: &[String]) -> String {
    let symbols = get_symbol_points(lines);
    let re_digits = Regex::new(r"(\d+)").unwrap();

    let mut total = 0;
    for (x, line) in lines.iter().enumerate() {
        for m in re_digits.find_iter(line) {
            let mut to_check: Vec<Point> = Vec::new();

            let value = m.as_str().parse::<i32>().unwrap();

            let current_line: i32 = x.try_into().unwrap();
            let start: i32 = m.start().try_into().unwrap();
            let end: i32 = m.end().try_into().unwrap();

            // Same line
            to_check.push(Point(current_line, start - 1));
            to_check.push(Point(current_line, end));

            // Previous and next lines
            for pos in start - 1..=end {
                to_check.push(Point(current_line - 1, pos));
                to_check.push(Point(current_line + 1, pos));
            }

            let has_symbol = to_check.iter().any(|p| symbols.contains(p));
            if has_symbol {
                total += value;
            }
        }
    }

    format!("{}", total)
}

fn get_symbol_points(lines: &[String]) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();

    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '0'..='9' => (),
                _ => {
                    let point = Point(x.try_into().unwrap(), y.try_into().unwrap());
                    result.insert(point);
                }
            };
        }
    }

    result
}
