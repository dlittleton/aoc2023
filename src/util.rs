use std::{fmt::Debug, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_INTEGERS: Regex = Regex::new(r"(-?\d+)").unwrap();
}

pub fn get_first_number<T>(s: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let m = RE_INTEGERS
        .find(s)
        .unwrap_or_else(|| panic!("Did not find number in string: {}", s));

    m.as_str().parse::<T>().unwrap()
}

pub fn get_all_numbers<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    RE_INTEGERS
        .find_iter(s)
        .map(|s| s.as_str().parse::<T>().unwrap())
        .collect()
}

pub fn extract_all_matches(pattern: &str, s: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap_or_else(|_| panic!("Bad Pattern: {}", pattern));
    return re.find_iter(s).map(|m| m.as_str().to_string()).collect();
}

pub fn combinations<T>(values: &[T]) -> impl Iterator<Item = (&T, &T)> {
    values[..]
        .iter()
        .enumerate()
        .flat_map(|(i, a)| values[(i + 1)..].iter().map(move |b| (a, b)))
}

pub fn combinations_3<T>(values: &[T]) -> impl Iterator<Item = (&T, &T, &T)> {
    values[..].iter().enumerate().flat_map(move |(i, a)| {
        values[(i + 1)..]
            .iter()
            .enumerate()
            .flat_map(move |(j, b)| values[(i + j + 2)..].iter().map(move |c| (a, b, c)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_empty() {
        let values: Vec<char> = Vec::new();
        let mut result = combinations(&values);

        assert!(result.next().is_none());
    }

    #[test]
    fn test_combinations() {
        let values: Vec<_> = "abcd".chars().collect();
        let combinations: Vec<_> = combinations(&values).collect();

        assert_eq!(combinations.len(), 6);
        assert_eq!(combinations[0], (&'a', &'b'));
        assert_eq!(combinations[1], (&'a', &'c'));
        assert_eq!(combinations[2], (&'a', &'d'));
        assert_eq!(combinations[3], (&'b', &'c'));
        assert_eq!(combinations[4], (&'b', &'d'));
        assert_eq!(combinations[5], (&'c', &'d'));
    }

    #[test]
    fn test_combinations_3() {
        let values: Vec<_> = "abcd".chars().collect();
        let combinations: Vec<_> = combinations_3(&values).collect();
        assert_eq!(combinations.len(), 4);
        assert_eq!(combinations[0], (&'a', &'b', &'c'));
        assert_eq!(combinations[1], (&'a', &'b', &'d'));
        assert_eq!(combinations[2], (&'a', &'c', &'d'));
        assert_eq!(combinations[3], (&'b', &'c', &'d'));
    }
}
