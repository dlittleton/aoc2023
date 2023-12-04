use aoc2023::util::get_first_number;

aoc2023::solver!(part1, part2);

#[derive(Default, Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

fn part1(lines: &[String]) -> String {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total = 0;

    for line in lines {
        let (game, spec) = line.split_once(":").unwrap();

        let game_number = get_first_number(game);

        let mut rounds = spec.split(';');
        let invalid = rounds.any(|r| {
            let round = parse_round(r);
            return round.red > max_red || round.green > max_green || round.blue > max_blue;
        });

        if !invalid {
            total += game_number
        }
    }

    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let mut total = 0;

    for line in lines {
        let (_, spec) = line.split_once(":").unwrap();

        let rounds = spec.split(';');
        let mut max_round = Round::default();

        for r in rounds {
            let round = parse_round(r);
            max_round.red = std::cmp::max(max_round.red, round.red);
            max_round.green = std::cmp::max(max_round.green, round.green);
            max_round.blue = std::cmp::max(max_round.blue, round.blue);
        }

        let power = max_round.red * max_round.blue * max_round.green;
        total += power
    }

    format!("{}", total)
}

fn parse_round(source: &str) -> Round {
    let mut round: Round = Round::default();

    for part in source.split(',').map(|s| s.trim()) {
        let (number, color) = part.split_once(' ').unwrap();
        let value = number.parse::<i32>().unwrap();

        match color {
            "red" => round.red = value,
            "green" => round.green = value,
            "blue" => round.blue = value,
            _ => panic!("Unexpected color {}", color),
        }
    }

    round
}
