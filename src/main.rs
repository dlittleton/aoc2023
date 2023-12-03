use log::{debug, info};
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let args = env::args().collect::<Vec<String>>();

    let day = &args[1];
    let input_file = &args[2];

    info!("Day {}, Files {}", day, input_file);

    let lines = read_input_file(input_file);
    day_1(&lines);
}

fn read_input_file(path: &String) -> Vec<String> {
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to open input file: {}", path));

    debug!("Input contents:\n{}", contents);

    return contents.split('\n').map(|s| s.to_string()).collect();
}

fn day_1(lines: &Vec<String>) {
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

    println!("{}", total);
}
