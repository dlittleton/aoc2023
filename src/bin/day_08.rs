use std::collections::HashMap;

use aoc2023::util::extract_all_matches;
use log::info;

aoc2023::solver!(part1, part2);

#[derive(Debug)]
struct MapEntry {
    node: String,
    left: String,
    right: String,
}

impl MapEntry {
    fn parse(line: &str) -> Self {
        let parts = extract_all_matches(r"[\dA-Z]{3}", line);

        match &parts[..] {
            [n, l, r] => Self {
                node: n.to_string(),
                left: l.to_string(),
                right: r.to_string(),
            },
            _ => panic!("Failed to parse map entry {}", line),
        }
    }
}

fn part1(lines: &[String]) -> String {
    let path = lines[0].chars().cycle();
    let mut map: HashMap<String, MapEntry> = HashMap::new();

    for line in &lines[2..] {
        let entry = MapEntry::parse(line);

        // Is this clone necessary?
        map.insert(entry.node.clone(), entry);
    }

    let mut steps = 0;
    let mut pos = "AAA";
    for dir in path {
        let entry = map.get(pos).unwrap();

        steps += 1;

        pos = match dir {
            'L' => &entry.left,
            'R' => &entry.right,
            _ => panic!("Unexpected direction: {}", dir),
        };

        if pos == "ZZZ" {
            break;
        }
    }

    format!("{}", steps)
}

fn part2(lines: &[String]) -> String {
    let path = lines[0].chars().cycle();
    let mut map: HashMap<String, MapEntry> = HashMap::new();

    for line in &lines[2..] {
        let entry = MapEntry::parse(line);

        // Is this clone necessary?
        map.insert(entry.node.clone(), entry);
    }

    let mut steps = 0;

    let mut positions: Vec<_> = map.keys().filter(|k| k.ends_with("A")).collect();
    info!("{:?}", positions);

    // Find the first 10 solutions for each
    let mut solutions: Vec<Vec<i32>> = positions.iter().map(|_| Vec::new()).collect();

    for dir in path {
        steps += 1;
        for (i, pos) in positions.iter_mut().enumerate() {
            let entry = map.get(*pos).unwrap();
            *pos = match dir {
                'L' => &entry.left,
                'R' => &entry.right,
                _ => panic!("Unexpected direction: {}", dir),
            };

            if pos.ends_with('Z') {
                info!("Position {}, Steps {}", i, steps);
                solutions[i].push(steps);
            }
        }

        if solutions.iter().all(|v| !v.is_empty()) {
            break;
        }
    }

    let least_steps: i64 = solutions
        .iter()
        .map(|v| v[0] as i64)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap();

    format!("{}", least_steps)
}
