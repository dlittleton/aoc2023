use std::{collections::VecDeque, ops::Range};

use aoc2023::util::get_all_numbers;
use log::info;

aoc2023::solver!(part1, part2);

#[derive(Debug)]
struct RangeMapping {
    source: u64,
    dest: u64,
    count: u64,
}

#[derive(Debug)]
struct MappingGroup {
    mappings: Vec<RangeMapping>,
}

impl MappingGroup {
    fn apply(&self, value: u64) -> u64 {
        for r in self.mappings.iter() {
            if (r.source..r.source + r.count).contains(&value) {
                let diff = value - r.source;
                let new_value = r.dest + diff;

                info!("Mapping {} to {}", value, new_value);
                return new_value;
            }
        }

        return value;
    }

    fn apply_range(&self, values: &mut Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut result = Vec::new();

        while let Some(v) = values.pop() {
            let mut matched = false;

            for r in self.mappings.iter() {
                let range_end = r.source + r.count;

                if v.end < r.source || v.start > range_end {
                    // No overlap
                    continue;
                }

                if v.start >= r.source && v.end <= range_end {
                    // Range is fully mapped
                    matched = true;
                    let offset = v.start - r.source;
                    let length = v.end - v.start;

                    result.push(r.dest + offset..r.dest + offset + length);
                    break;
                }

                if v.start < r.source && v.end > range_end {
                    // Value spans entire range
                    matched = true;
                    result.push(r.dest..r.dest + r.count);

                    let leading_len = r.source - v.start;

                    values.push(v.start..v.start + leading_len);
                    values.push(range_end..v.end);
                    break;
                }

                if v.start < r.source && v.end > r.source {
                    // Beginning overlaps
                    matched = true;

                    let mapped_len = v.end - r.source;
                    result.push(r.dest..r.dest + mapped_len);

                    let leading_len = r.source - v.start;
                    values.push(v.start..v.start + leading_len);
                    break;
                }

                if v.end > range_end && v.start < range_end {
                    matched = true;
                    let offset = v.start - r.source;
                    let mapped_len = range_end - v.start;

                    result.push(r.dest + offset..r.dest + offset + mapped_len);
                    values.push(range_end..v.end);
                    break;
                }
            }

            if !matched {
                result.push(v);
            }
        }

        result
    }
}

fn part1(lines: &[String]) -> String {
    let mut input = lines.iter();
    let values: Vec<u64> = get_all_numbers(input.next().unwrap());

    // Discard empty line
    input.next();

    let mappings = build_mappings(&mut input);

    let min = values
        .iter()
        .map(|v| {
            let mut mapped_value = *v;
            for group in &mappings[..] {
                mapped_value = group.apply(mapped_value);
            }
            mapped_value
        })
        .min()
        .unwrap();

    format!("{}", min)
}

fn part2(lines: &[String]) -> String {
    let mut input = lines.iter();
    let mut values: VecDeque<u64> = VecDeque::from(get_all_numbers(input.next().unwrap()));

    // Discard empty line
    input.next();

    let mappings = build_mappings(&mut input);

    let mut ranges: Vec<_> = Vec::new();
    while let (Some(start), Some(count)) = (values.pop_front(), values.pop_front()) {
        ranges.push(start..start + count);
    }

    for group in mappings {
        ranges = group.apply_range(&mut ranges);
    }

    let min = ranges.iter().map(|r| r.start).min().unwrap();

    format!("{}", min)
}

fn build_mappings<'a, T>(lines: &mut T) -> Vec<MappingGroup>
where
    T: Iterator<Item = &'a String>,
{
    let mut peekable_lines = lines.peekable();
    let mut result = Vec::new();

    while peekable_lines.peek().is_some() {
        result.push(MappingGroup {
            mappings: parse_mapping_group(&mut peekable_lines),
        })
    }

    result
}

fn parse_mapping_group<'a, T>(lines: &mut T) -> Vec<RangeMapping>
where
    T: Iterator<Item = &'a String>,
{
    lines
        .into_iter()
        .skip(1)
        .map(|line| parse_mapping(line))
        .take_while(|m| m.is_some())
        .flatten()
        .collect()
}

fn parse_mapping(line: &str) -> Option<RangeMapping> {
    match get_all_numbers(line)[..] {
        [dest, source, count] => Some(RangeMapping {
            source,
            dest,
            count,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_group() -> Vec<MappingGroup> {
        let lines = &vec!["header", "10 75 25", "200 100 25", "1000 2000 100"];
        let temp: Vec<_> = lines.iter().map(|l| l.to_string()).collect();
        build_mappings(&mut temp.iter())
    }

    #[test]
    fn test_parse_mapping() {
        let parsed = parse_mapping("50 92 2").unwrap();
        assert_eq!(parsed.dest, 50);
        assert_eq!(parsed.source, 92);
        assert_eq!(parsed.count, 2);
    }

    #[test]
    fn test_apply() {
        let group = create_test_group();
        assert_eq!(34, group[0].apply(99));
        assert_eq!(200, group[0].apply(100));
        assert_eq!(300, group[0].apply(300));
    }

    #[test]
    fn test_apply_range_unmapped() {
        let group = create_test_group();

        let mut values = vec![0..10];
        let result = group[0].apply_range(&mut values);

        assert_eq!(1, result.len());
        assert_eq!(0..10, result[0]);
    }

    #[test]
    fn test_apply_range_left() {
        let group = create_test_group();

        let mut values = vec![0..10, 65..85];
        let mut result = group[0].apply_range(&mut values);
        result.sort_by(|l, r| l.start.cmp(&r.start));

        assert_eq!(3, result.len());
        assert_eq!(0..10, result[0]);
        assert_eq!(10..20, result[1]); // Mapped value
        assert_eq!(65..75, result[2]); // Unmapped portion
    }

    #[test]
    fn test_apply_range_right() {
        let group = create_test_group();

        let mut values = vec![0..10, 115..135];
        let mut result = group[0].apply_range(&mut values);
        result.sort_by(|l, r| l.start.cmp(&r.start));

        assert_eq!(3, result.len());
        assert_eq!(0..10, result[0]);
        assert_eq!(125..135, result[1]); // Mapped value
        assert_eq!(215..225, result[2]); // Unmapped portion
    }

    #[test]
    fn test_apply_range_value_contained_in_range() {
        let group = create_test_group();

        let mut values = vec![85..95];
        let result = group[0].apply_range(&mut values);

        assert_eq!(1, result.len());
        assert_eq!(20..30, result[0]);
    }

    #[test]
    fn test_apply_range_range_contained_in_value() {
        let group = create_test_group();

        let mut values = vec![0..10, 1990..2110];
        let mut result = group[0].apply_range(&mut values);
        result.sort_by(|l, r| l.start.cmp(&r.start));

        assert_eq!(4, result.len());
        assert_eq!(0..10, result[0]);
        assert_eq!(1000..1100, result[1]); // Mapped value
        assert_eq!(1990..2000, result[2]); // Leading portion
        assert_eq!(2100..2110, result[3]); // Trailing portion
    }
}
