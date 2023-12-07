use std::{collections::HashMap, iter};

use aoc2023::util::get_first_number;
use log::{info, log_enabled};

aoc2023::solver!(part1);

const CARD_RANKS: &str = "23456789TJQKA";

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    bid: i32,
    cards: Vec<char>,
    kind: HandKind,
}

impl Hand {
    fn parse(line: &String) -> Self {
        let (cards_str, bid_str) = line.split_once(' ').unwrap();

        let cards: Vec<_> = cards_str.chars().collect();
        let counts = count_cards(cards_str);
        let bid = get_first_number(bid_str);

        let mut ranks: Vec<_> = counts
            .into_values()
            .chain(iter::repeat(0))
            .take(5)
            .collect();
        ranks.sort();

        let kind = match ranks[..] {
            [0, 0, 0, 0, 5] => HandKind::FiveOfAKind,
            [0, 0, 0, 1, 4] => HandKind::FourOfAKind,
            [0, 0, 0, 2, 3] => HandKind::FullHouse,
            [0, 0, 1, 1, 3] => HandKind::ThreeOfAKind,
            [0, 0, 1, 2, 2] => HandKind::TwoPair,
            [0, 1, 1, 1, 2] => HandKind::OnePair,
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            _ => panic!("Unmatched hand!"),
        };

        Hand { bid, cards, kind }
    }

    fn key_by_hand_order(&self) -> (HandKind, usize, usize, usize, usize, usize) {
        let values: Vec<_> = self
            .cards
            .iter()
            .map(|c| CARD_RANKS.find(*c).unwrap())
            .collect();

        (
            self.kind, values[0], values[1], values[2], values[3], values[4],
        )
    }
}

fn count_cards(cards: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();

    for c in cards.chars() {
        if result.contains_key(&c) {
            *result.get_mut(&c).unwrap() += 1;
        } else {
            result.insert(c, 1);
        }
    }

    result
}

fn part1(lines: &[String]) -> String {
    let mut hands: Vec<_> = lines.iter().map(|l| Hand::parse(l)).collect();
    hands.sort_by_key(|h| h.key_by_hand_order());

    if log_enabled!(log::Level::Info) {
        for h in &hands[..] {
            info!("{:?}", h);
        }
    }

    let score = score_hands(&hands);
    format!("{}", score)
}

fn score_hands(hands: &[Hand]) -> i32 {
    let mut total: i32 = 0;

    for (i, h) in hands.iter().enumerate() {
        total += h.bid * (i as i32 + 1)
    }

    total
}
