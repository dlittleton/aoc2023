use std::collections::VecDeque;

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

    let min = ranges
        .iter()
        .flat_map(|m| m.clone().into_iter())
        .map(|v| {
            let mut mapped_value = v;
            for group in &mappings[..] {
                mapped_value = group.apply(mapped_value);
            }
            mapped_value
        })
        .min()
        .unwrap();

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
