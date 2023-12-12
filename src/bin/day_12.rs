use lazy_static::lazy_static;
use regex::Regex;

use aoc2023::util::get_all_numbers;

lazy_static! {
    static ref RE_BROKEN: Regex = Regex::new(r"(#+)").unwrap();
}

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let total: usize = lines.iter().map(count_variations).sum();
    format!("{}", total)
}

fn count_variations(line: &String) -> usize {
    let (spec, values) = line.split_once(' ').unwrap();

    let counts: Vec<_> = get_all_numbers(values);

    let springs: Vec<_> = spec.chars().collect();
    let valid = permute(&springs).filter(|p| is_valid(p, &counts)).count();

    return valid;
}

fn permute(springs: &[char]) -> Box<dyn Iterator<Item = String>> {
    let mut current: Vec<_> = Vec::new();
    if let [head, tail @ ..] = springs {
        if *head == '?' {
            current.push("#".to_string());
            current.push(".".to_string());
        } else {
            current.push(format!("{}", head))
        }

        if !tail.is_empty() {
            let children = permute(tail);
            let temp = children
                .flat_map(move |child| {
                    current
                        .clone()
                        .into_iter()
                        .map(move |cur| format!("{}{}", cur, child))
                })
                .into_iter();
            return Box::new(temp);
        }
    }

    Box::new(current.into_iter())
}

fn is_valid(permutation: &String, counts: &[usize]) -> bool {
    let actual: Vec<_> = RE_BROKEN.find_iter(permutation).map(|m| m.len()).collect();
    actual == counts
}
