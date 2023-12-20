use std::collections::HashMap;

use regex::Regex;
aoc2023::solver!(part1);

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
