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
struct Link {
    node: usize,
    connection: usize,
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: Vec<Vec<Link>>,
    connections: Vec<Connection<'a>>,
}

impl<'a> Graph<'a> {
    fn new(connections: Vec<Connection<'a>>) -> Self {
        let names: HashSet<_> = connections.iter().flat_map(|c| vec![c.0, c.1]).collect();
        let node_map: HashMap<_, _> = names.iter().enumerate().map(|(i, n)| (n, i)).collect();

        let mut nodes = Vec::with_capacity(node_map.len());
        for _ in 0..node_map.len() {
            nodes.push(Vec::new());
        }

        connections.iter().enumerate().for_each(|(i, c)| {
            let a = node_map.get(&c.0).unwrap();
            let b = node_map.get(&c.1).unwrap();

            nodes[*a].push(Link {
                node: *b,
                connection: i,
            });
            nodes[*b].push(Link {
                node: *a,
                connection: i,
            });
        });

        Self { nodes, connections }
    }
}

fn part1(lines: &[String]) -> String {
    let connections: Vec<_> = parse(lines).collect();
    info!("Connection count: {}", connections.len());

    let graph = Graph::new(connections);

    let (a, b) = try_partion(&graph);

    let total = a * b;

    format!("{}", total)
}

fn parse(lines: &[String]) -> impl Iterator<Item = Connection> {
    lines.iter().flat_map(|line| {
        let (source, dest) = line.split_once(": ").unwrap();

        dest.split(" ").map(move |d| Connection(source, d))
    })
}

fn try_partion<'a>(graph: &Graph<'a>) -> (usize, usize) {
    let start = SystemTime::now();

    let connection_idx: Vec<_> = (0..graph.connections.len()).collect();

    let result = combinations_3(&connection_idx)
        .enumerate()
        .find_map(|(i, (a, b, c))| {
            let mut visited = vec![false; graph.nodes.len()];

            let mut to_visit = vec![0];

            if i % 100000 == 0 {
                let duration = SystemTime::now()
                    .duration_since(start)
                    .unwrap()
                    .as_secs_f64();
                let ips = i as f64 / duration;
                info!("Iteration {}, {:.2} iterations per second", i, ips);
            }

            // Visit every node connected to the start node.
            while !to_visit.is_empty() {
                let current = to_visit.pop().unwrap();
                visited[current] = true;

                graph.nodes[current].iter().for_each(|link| {
                    if link.connection != *a
                        && link.connection != *b
                        && link.connection != *c
                        && !visited[link.node]
                    {
                        to_visit.push(link.node);
                    }
                })
            }

            let connected = visited.iter().filter(|&v| *v).count();
            if connected != visited.len() {
                let remaining = visited.len() - connected;
                info!("Found two partitions {}, {}", connected, remaining);
                return Some((connected, remaining));
            }

            None
        })
        .unwrap();

    return result;
}
