use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(\d+)").unwrap();
}

pub fn get_first_number(s: &str) -> i32 {
    let m = RE_DIGITS
        .find(s)
        .unwrap_or_else(|| panic!("Did not find number in string: {}", s));

    m.as_str().parse::<i32>().unwrap()
}

pub fn get_all_numbers(s: &str) -> Vec<i32> {
    RE_DIGITS
        .find_iter(s)
        .map(|s| s.as_str().parse::<i32>().unwrap())
        .collect()
}
