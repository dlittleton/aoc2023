use std::{fmt::Debug, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(\d+)").unwrap();
}

pub fn get_first_number<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let m = RE_DIGITS
        .find(s)
        .unwrap_or_else(|| panic!("Did not find number in string: {}", s));

    m.as_str().parse::<T>().unwrap()
}

pub fn get_all_numbers<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    RE_DIGITS
        .find_iter(s)
        .map(|s| s.as_str().parse::<T>().unwrap())
        .collect()
}

pub fn extract_all_matches(pattern: &str, s: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap_or_else(|_| panic!("Bad Pattern: {}", pattern));
    return re.find_iter(s).map(|m| m.as_str().to_string()).collect();
}
