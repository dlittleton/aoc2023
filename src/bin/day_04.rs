use aoc2023::util::{self, get_first_number};
use std::collections::HashSet;

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let mut total = 0;
    for line in lines {
        let (_, card) = line.split_once(':').unwrap();

        let winners = count_winning_numbers(card);
        if winners > 0 {
            total += 2_i32.pow((winners - 1).try_into().unwrap());
        }
    }

    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let mut counts: Vec<_> = lines.iter().map(|_| 1).collect();
    for line in lines {
        let (card, values) = line.split_once(':').unwrap();
        let card_num: usize = get_first_number(card);
        let count = count_winning_numbers(values) as usize;

        let copies = counts[card_num - 1];
        for i in card_num..card_num + count {
            counts[i] += copies
        }
    }

    let total: i32 = counts.iter().sum();

    format!("{}", total)
}

fn count_winning_numbers(card: &str) -> i32 {
    let (winning, have) = card.split_once('|').unwrap();

    let winning_set: HashSet<i32> = HashSet::from_iter(util::get_all_numbers(winning));
    let have_set: HashSet<i32> = HashSet::from_iter(util::get_all_numbers(have));

    winning_set
        .intersection(&have_set)
        .count()
        .try_into()
        .unwrap()
}
