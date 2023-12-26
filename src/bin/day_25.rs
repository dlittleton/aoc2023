use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use aoc2023::util::combinations_3;
use log::info;

aoc2023::solver!(part1);

#[derive(Debug)]
struct Connection<'a>(&'a str, &'a str);

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        let nodes = HashMap::new();

        Self { nodes }
    }

    fn from_connections(connections: &[Connection<'a>]) -> Self {
        let mut graph = Self::new();

        connections.iter().for_each(|c| graph.add_connection(c));

        graph
    }

    fn add_connection(&mut self, connection: &Connection<'a>) {
        self.nodes
            .entry(&connection.0)
            .or_insert(HashSet::new())
            .insert(connection.1);

        self.nodes
            .entry(&connection.1)
            .or_insert(HashSet::new())
            .insert(connection.0);
    }

    fn remove_connection(&mut self, connection: &Connection<'a>) {
        self.nodes
            .get_mut(connection.0)
            .unwrap()
            .remove(connection.1);

        self.nodes
            .get_mut(connection.1)
            .unwrap()
            .remove(connection.0);
    }
}

fn part1(lines: &[String]) -> String {
    let connections: Vec<_> = parse(lines).collect();
    info!("Connection count: {}", connections.len());

    let mut graph = Graph::from_connections(&connections);

    let (a, b) = try_partion(&connections, &mut graph);

    let total = a * b;

    format!("{}", total)
}

fn parse(lines: &[String]) -> impl Iterator<Item = Connection> {
    lines.iter().flat_map(|line| {
        let (source, dest) = line.split_once(": ").unwrap();

        dest.split(" ").map(move |d| Connection(source, d))
    })
}

fn try_partion<'a>(connections: &[Connection<'a>], graph: &mut Graph<'a>) -> (usize, usize) {
    let all_nodes: Vec<_> = graph.nodes.keys().cloned().collect();

    let start = SystemTime::now();

    combinations_3(connections)
        .enumerate()
        .find_map(|(i, (a, b, c))| {
            graph.remove_connection(a);
            graph.remove_connection(b);
            graph.remove_connection(c);

            let mut seen: HashSet<_> = HashSet::new();

            if i % 100000 == 0 {
                let duration = SystemTime::now()
                    .duration_since(start)
                    .unwrap()
                    .as_secs_f64();
                let ips = i as f64 / duration;
                info!("Iteration {}, {:.2} iterations per second", i, ips);
            }

            let mut to_visit = vec![all_nodes[0]];
            while !to_visit.is_empty() {
                let current = to_visit.pop().unwrap();
                if !seen.contains(current) {
                    seen.insert(current);

                    graph
                        .nodes
                        .get(current)
                        .unwrap()
                        .iter()
                        .for_each(|n| to_visit.push(n));
                }
            }

            if seen.len() != all_nodes.len() {
                let s_count = seen.len();
                let remaining = all_nodes.len().abs_diff(s_count);
                info!("Multiple partitions of size {}, {}", s_count, remaining);
                return Some((s_count, remaining));
            }

            graph.add_connection(a);
            graph.add_connection(b);
            graph.add_connection(c);
            None
        })
        .unwrap()
}
