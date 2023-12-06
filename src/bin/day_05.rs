use aoc2023::util::get_all_numbers;
use log::info;

aoc2023::solver!(part1);

#[derive(Debug)]
struct RangeMapping {
    source: i32,
    dest: i32,
    count: i32,
}

fn part1(lines: &[String]) -> String {
    let mut mappings = lines.iter().peekable();
    let mut values = get_all_numbers(mappings.next().unwrap());

    // Discard empty line
    mappings.next();

    while mappings.peek().is_some() {
        values = apply_mapping(&values, &mut mappings);
        info!("Values is now {:?}", values)
    }

    let min = values.iter().min().unwrap();
    format!("{}", min)
}

fn apply_mapping<'a, T>(values: &[i32], lines: &mut T) -> Vec<i32>
where
    T: Iterator<Item = &'a String>,
{
    let ranges: Vec<_> = lines
        .into_iter()
        .skip(1)
        .map(|line| parse_mapping(line))
        .take_while(|m| m.is_some())
        .flatten()
        .collect();

    values
        .into_iter()
        .map(|v| {
            for r in &ranges[..] {
                if (r.source..r.source + r.count).contains(v) {
                    let diff = v - r.source;
                    let new_value = r.dest + diff;

                    info!("Mapping {} to {}", v, new_value);
                    return new_value;
                }
            }

            *v
        })
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
