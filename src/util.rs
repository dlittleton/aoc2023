use regex::Regex;

pub fn get_first_number(s: &str) -> i32 {
    let re_digits = Regex::new(r"(\d+)").unwrap();

    let m = re_digits
        .find(s)
        .unwrap_or_else(|| panic!("Did not find number in string: {}", s));

    m.as_str().parse::<i32>().unwrap()
}
