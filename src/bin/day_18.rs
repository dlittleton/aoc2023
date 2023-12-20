use log::info;
use regex::Regex;

aoc2023::solver!(part1, part2);

#[derive(Debug)]
struct Point(i64, i64);
type LinePath = Vec<Point>;

fn part1(lines: &[String]) -> String {
    let (path, length) = parse_path(lines);
    let area = calculate_area(&path);

    // Enclosed area + path area + 1 for origin.
    let total = area + (length / 2) + 1;
    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let (path, length) = parse_path_hex(lines);
    let area = calculate_area(&path);

    // Enclosed area + path area + 1 for origin.
    let total = area + (length / 2) + 1;
    format!("{}", total)
}

fn parse_path(lines: &[String]) -> (LinePath, i64) {
    let re_path = Regex::new(r"([RDLU]) (\d+)").unwrap();

    let mut result = LinePath::new();
    result.push(Point(0, 0));

    let mut overall_len = 0;

    for line in lines {
        for c in re_path.captures_iter(line) {
            let previous = result.last().unwrap();

            let dir = c.get(1).unwrap().as_str();
            let count = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
            overall_len += count;

            let next_point = match dir {
                "R" => Point(previous.0 + count, previous.1),
                "D" => Point(previous.0, previous.1 - count),
                "L" => Point(previous.0 - count, previous.1),
                "U" => Point(previous.0, previous.1 + count),
                _ => panic!("Unexpected direction {}", dir),
            };

            result.push(next_point);
        }
    }

    info!("Overall length {}", overall_len);

    (result, overall_len)
}

fn parse_path_hex(lines: &[String]) -> (LinePath, i64) {
    let re_path = Regex::new(r"#(\w{5})(\d)").unwrap();

    let mut result = LinePath::new();
    result.push(Point(0, 0));

    let mut overall_len = 0;

    for line in lines {
        for c in re_path.captures_iter(line) {
            let previous = result.last().unwrap();

            let dir = c.get(2).unwrap().as_str();
            let count = i64::from_str_radix(c.get(1).unwrap().as_str(), 16).unwrap();
            overall_len += count;

            let next_point = match dir {
                "0" => Point(previous.0 + count, previous.1),
                "1" => Point(previous.0, previous.1 - count),
                "2" => Point(previous.0 - count, previous.1),
                "3" => Point(previous.0, previous.1 + count),
                _ => panic!("Unexpected direction {}", dir),
            };

            result.push(next_point);
        }
    }

    info!("Overall length {}", overall_len);

    (result, overall_len)
}

/*
 * Calculate area using the shoelace formula
 *
 * https://en.wikipedia.org/wiki/Shoelace_formula
 */
fn calculate_area(path: &LinePath) -> i64 {
    let mut total = 0;
    // Origin is duplicated in first and last position
    for i in 0..path.len() - 1 {
        let current = &path[i];
        let next = &path[i + 1];

        total += (current.1 + next.1) * (current.0 - next.0)
    }

    (total / 2).abs()
}
