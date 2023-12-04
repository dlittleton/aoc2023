use log::debug;
use regex::Regex;

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let re_first = Regex::new(r"^[^\d]*(\d)").unwrap();
    let re_last = Regex::new(r".*(\d)[^\d]*$").unwrap();

    let mut total = 0;
    for l in lines {
        let first_digit = re_first.captures(l).unwrap();
        let last_digit = re_last.captures(l).unwrap();

        let digits = format!("{}{}", &first_digit[1], &last_digit[1]);
        let numeric = digits.parse::<i32>().unwrap();

        total += numeric;
    }

    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();

    let mut total = 0;
    for l in lines {
        let mut digits: Vec<i32> = vec![];
        let mut index = 0;
        while let Some(m) = re.find_at(l, index) {
            index = m.start() + 1;
            let capture = m.as_str();
            let value = match capture {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "zero" => 0,
                _ => capture.parse::<i32>().unwrap(),
            };

            digits.push(value);
        }

        let first_digit = digits.first().unwrap();
        let second_digit = digits.last().unwrap();

        debug!("{}{}", first_digit, second_digit);

        total += (10 * first_digit) + second_digit;
    }

    format!("{}", total)
}
