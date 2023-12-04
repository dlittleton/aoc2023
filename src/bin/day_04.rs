aoc2023::solver!(part1);
use aoc2023::util;
use std::collections::HashSet;

fn part1(lines: &[String]) -> String {
    let mut total = 0;
    for line in lines {
        let (_, card) = line.split_once(':').unwrap();

        total += count_winning_numbers(card);
    }

    format!("{}", total)
}

fn count_winning_numbers(card: &str) -> i32 {
    let (winning, have) = card.split_once('|').unwrap();

    let winning_set: HashSet<i32> = HashSet::from_iter(util::get_all_numbers(winning));
    let have_set: HashSet<i32> = HashSet::from_iter(util::get_all_numbers(have));

    let nums = winning_set.intersection(&have_set).count();

    if nums == 0 {
        0
    } else {
        2_i32.pow((nums - 1).try_into().unwrap())
    }
}
