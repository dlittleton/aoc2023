use std::collections::HashMap;

use log::info;
use regex::Regex;
aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let mut line_iter = lines.into_iter();
    let rules = parse_rules(&mut line_iter);
    let parts = parse_parts(&mut line_iter);

    let total: i32 = parts
        .iter()
        .filter(|p| inspect(&rules, p))
        .map(|p| p.values().sum::<i32>())
        .sum();

    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let mut line_iter = lines.into_iter();
    let rules = parse_rules(&mut line_iter);

    let total = count_possibilities(&rules);

    format!("{}", total)
}

#[derive(Debug)]
struct Rule {
    property: String,
    op: String,
    value: i32,
    target: String,
}

impl Rule {
    fn parse(spec: &str) -> Self {
        let re_rule = Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();

        if let Some(capture) = re_rule.captures(spec) {
            let property = capture.get(1).unwrap().as_str().to_string();
            let op = capture.get(2).unwrap().as_str().to_string();
            let value = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let target = capture.get(4).unwrap().as_str().to_string();

            Rule {
                property,
                op,
                value,
                target,
            }
        } else {
            Rule {
                property: "".to_string(),
                op: "".to_string(),
                value: 0,
                target: spec.to_string(),
            }
        }
    }

    fn apply(&self, part: &Part) -> Option<&str> {
        if self.op.is_empty() {
            // Default rule
            return Some(&self.target);
        }

        let actual = *part.get(&self.property).unwrap();
        if self.op == ">" && actual > self.value || self.op == "<" && actual < self.value {
            return Some(&self.target);
        } else {
            return None;
        }
    }
}

type RuleMap = HashMap<String, Vec<Rule>>;
type Part = HashMap<String, i32>;
type PartRange = HashMap<String, std::ops::Range<i32>>;

fn parse_rules<'a, T>(lines: &mut T) -> RuleMap
where
    T: Iterator<Item = &'a String>,
{
    let re_rule_definition = Regex::new(r"(\w+)\{([^}]*)\}").unwrap();

    lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let capture = re_rule_definition.captures(line).unwrap();

            let name = capture.get(1).unwrap().as_str().to_string();

            let rules: Vec<_> = capture
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(|r| Rule::parse(r))
                .collect();

            (name, rules)
        })
        .collect()
}

fn parse_parts<'a, T>(lines: &mut T) -> Vec<Part>
where
    T: Iterator<Item = &'a String>,
{
    let re_part = Regex::new(r"(\w+)=(\d+)").unwrap();

    lines
        .map(|line| {
            re_part
                .captures_iter(line)
                .map(|c| {
                    let prop = c.get(1).unwrap().as_str().to_string();
                    let value = c.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    (prop, value)
                })
                .collect()
        })
        .collect()
}

fn inspect(rules: &RuleMap, part: &Part) -> bool {
    let mut name = "in";

    loop {
        let rule = rules.get(name).unwrap();

        let next = rule.iter().find_map(|r| r.apply(part)).unwrap();
        match next {
            "R" => return false,
            "A" => return true,
            x => name = x,
        };
    }
}

fn count_possibilities(rules: &RuleMap) -> usize {
    let mut all = PartRange::new();
    all.insert("x".to_string(), 1..4001);
    all.insert("m".to_string(), 1..4001);
    all.insert("a".to_string(), 1..4001);
    all.insert("s".to_string(), 1..4001);

    let mut to_check: Vec<_> = Vec::new();
    to_check.push(("in", all.clone()));

    let mut total = 0;

    while !to_check.is_empty() {
        let (name, mut pr) = to_check.pop().unwrap();

        if name == "R" {
            continue;
        }

        if name == "A" {
            info!("Accepted {:?}", pr);

            total += pr
                .values()
                .map(|v| v.clone().count() as usize)
                .product::<usize>();
            continue;
        }

        let rule = rules.get(name).unwrap();

        for r in rule {
            // Default rule, move everything
            if r.op.is_empty() {
                to_check.push((&r.target, pr.clone()));
                continue;
            }

            let prop_range = pr.get(&r.property).unwrap();
            if !prop_range.contains(&r.value) {
                continue;
            }

            if r.op == "<" {
                let mut new_parts = pr.clone();
                *new_parts.get_mut(&r.property).unwrap() = prop_range.start..r.value;
                to_check.push((&r.target, new_parts));

                *pr.get_mut(&r.property).unwrap() = r.value..prop_range.end;
            } else {
                let mut new_parts = pr.clone();
                *new_parts.get_mut(&r.property).unwrap() = r.value + 1..prop_range.end;
                to_check.push((&r.target, new_parts));

                *pr.get_mut(&r.property).unwrap() = prop_range.start..r.value + 1;
            }
        }
    }

    total
}
