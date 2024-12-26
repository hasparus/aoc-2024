use std::collections::{HashMap, HashSet};

use crate::parse_input::{Computer, Connection};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Triangle {
    computers: [Computer; 3],
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.computers[0], self.computers[1], self.computers[2]
        )
    }
}

impl Triangle {
    fn new(a: &Computer, b: &Computer, c: &Computer) -> Self {
        let mut computers = [*a, *b, *c];
        computers.sort();
        Self { computers }
    }
}

pub fn solve(input: &str) -> usize {
    let connections = input
        .lines()
        .map(|line| {
            line.trim()
                .parse::<Connection>()
                .unwrap_or_else(|_| panic!("Invalid connection format {line}"))
        })
        .collect::<Vec<_>>();

    let computers: HashSet<Computer> = HashSet::from_iter(
        connections
            .iter()
            .flat_map(|connection| [connection.0, connection.1]),
    );

    let computers = computers.into_iter().collect::<Vec<_>>();

    let mut graph: HashMap<Computer, Vec<Computer>> = HashMap::new();
    for Connection(left, right) in connections {
        graph.entry(left).or_default().push(right);
        graph.entry(right).or_default().push(left);
    }

    let historian_suspects = computers
        .iter()
        .filter(|computer| computer.can_be_chief_historian())
        .collect::<Vec<_>>();

    let mut triangles = HashSet::new();

    for computer in historian_suspects {
        let neighbors = graph
            .get(computer)
            .expect("Historian suspect not found in graph");

        for neighbor in neighbors {
            let neighbor_neighbors = graph.get(neighbor).expect("Neighbor not found in graph");

            for neighbor_neighbor in neighbor_neighbors {
                if neighbor_neighbor == computer {
                    continue;
                }

                let neighbor_neighbor_neighbors = graph
                    .get(neighbor_neighbor)
                    .expect("Neighbor neighbor not found in graph");

                if neighbor_neighbor_neighbors.contains(computer) {
                    triangles.insert(Triangle::new(computer, neighbor, neighbor_neighbor));
                }
            }
        }
    }

    triangles.len()
}
