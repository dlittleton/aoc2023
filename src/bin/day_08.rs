use std::collections::HashMap;

use aoc2023::util::extract_all_matches;

aoc2023::solver!(part1);

#[derive(Debug)]
struct MapEntry {
    node: String,
    left: String,
    right: String,
}

impl MapEntry {
    fn parse(line: &str) -> Self {
        let parts = extract_all_matches("[A-Z]{3}", line);

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
