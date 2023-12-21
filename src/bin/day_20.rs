use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use log::{debug, info};

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let mut modules: HashMap<_, _> = lines
        .iter()
        .map(Module::parse)
        .map(|m| (m.name, m))
        .collect();

    initialize_inputs(&mut modules);
    let (low, high) = press(&mut modules, 1000);
    info!("Low {}, High {}", low, high);
    let total = low * high;
    format!("{}", total)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseKind {
    Low,
    High,
}

impl Display for PulseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PulseKind::Low => write!(f, "{}", "low"),
            PulseKind::High => write!(f, "{}", "high"),
        }
    }
}

struct Pulse<'a> {
    source: &'a str,
    destination: &'a str,
    kind: PulseKind,
}

impl<'a> Display for Pulse<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.source, self.kind, self.destination)
    }
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    op: Option<&'a str>,
    destinations: Vec<&'a str>,
    state: PulseKind,
    inputs: HashMap<&'a str, PulseKind>,
}

impl<'a> Module<'a> {
    fn parse(line: &'a String) -> Self {
        let (mut name, targets) = line.split_once(" -> ").unwrap();
        let mut op = None;

        if name.starts_with("%") || name.starts_with("&") {
            let (a, b) = name.split_at(1);
            op = Some(a);
            name = b;
        }

        let destinations = targets.split(", ").collect();
        let state = PulseKind::Low;
        let inputs = HashMap::new();

        Self {
            name,
            op,
            destinations,
            state,
            inputs,
        }
    }

    fn apply(&mut self, input: &Pulse<'a>) -> Option<Vec<Pulse<'a>>> {
        if self.op.is_none() {
            // Broadcaster
            return Some(self.generate(input.kind));
        }

        let op = self.op.unwrap();
        if op == "%" {
            // Flip flop
            if input.kind == PulseKind::High {
                return None;
            }

            self.state = match self.state {
                PulseKind::Low => PulseKind::High,
                PulseKind::High => PulseKind::Low,
            };

            return Some(self.generate(self.state));
        }

        if op == "&" {
            // Conjunction
            self.inputs.insert(input.source, input.kind);

            let mut kind = PulseKind::High;

            if self.inputs.values().all(|v| *v == PulseKind::High) {
                kind = PulseKind::Low;
            }

            return Some(self.generate(kind));
        }

        None
    }

    fn generate(&self, kind: PulseKind) -> Vec<Pulse<'a>> {
        self.destinations
            .iter()
            .map(|d| Pulse {
                source: self.name,
                destination: *d,
                kind,
            })
            .collect()
    }
}

fn initialize_inputs<'a>(modules: &mut HashMap<&'a str, Module<'a>>) {
    info!("Initializing inputs for conjunction modules.");

    let conjunctions: Vec<_> = modules
        .values()
        .filter(|m| m.op.unwrap_or("") == "&")
        .map(|m| m.name)
        .collect();

    for c in conjunctions.iter() {
        let inputs: Vec<_> = modules
            .values()
            .filter(|m| m.destinations.contains(c))
            .map(|m| m.name)
            .collect();

        info!("Conjunction {} has inputs {:?}", c, inputs);

        let module = modules.get_mut(c).unwrap();
        for i in inputs.iter() {
            module.inputs.insert(i, PulseKind::Low);
        }
    }
}

fn press(modules: &mut HashMap<&str, Module>, count: usize) -> (usize, usize) {
    let mut signals: VecDeque<_> = VecDeque::new();
    info!("Pressing the button {} times.", count);

    let mut low: usize = 0;
    let mut high: usize = 0;
    for _ in 0..count {
        signals.push_back(Pulse {
            source: "button",
            destination: "broadcaster",
            kind: PulseKind::Low,
        });

        while !signals.is_empty() {
            let current = signals.pop_front().unwrap();
            debug!("{}", current);

            match current.kind {
                PulseKind::Low => low += 1,
                PulseKind::High => high += 1,
            };

            if let Some(result) = modules
                .get_mut(current.destination)
                .and_then(|m| m.apply(&current))
            {
                signals.extend(result);
            }
        }
    }

    (low, high)
}
