advent_of_code::solution!(23);

use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug)]
struct Pair {
    name1: String,
    name2: String,
}
impl Pair {
    fn new(name1: String, name2: String) -> Self {
        let (c1, c2) = if name1 < name2 {
            (name1, name2)
        } else {
            (name2, name1)
        };
        Self {
            name1: c1,
            name2: c2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge {
    others: BTreeSet<String>,
}
impl From<Vec<String>> for Edge {
    fn from(others: Vec<String>) -> Self {
        Self {
            others: others.into_iter().collect(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            if let [c1, c2] = &line
                .split("-")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[..]
            {
                Pair::new(c1.to_string(), c2.to_string())
            } else {
                panic!("Invalid input");
            }
        })
        .collect()
}

// This is a trivial function to write, but might aid with generalisation
fn find_twoway_connections(pairs: &[Pair]) -> HashMap<String, HashSet<Edge>> {
    let mut edges: HashMap<String, HashSet<Edge>> = HashMap::new();
    pairs.iter().for_each(|pair| {
        let c1 = Edge::from(vec![pair.name2.clone()]);
        edges.entry(pair.name1.clone()).or_default().insert(c1);
        let c2 = Edge::from(vec![pair.name1.clone()]);
        edges.entry(pair.name2.clone()).or_default().insert(c2);
    });
    edges
}

fn find_nway_connections(
    pairs: &[Pair],
    nminus1way_connections: HashMap<String, HashSet<Edge>>,
) -> HashMap<String, HashSet<Edge>> {
    let mut edges: HashMap<String, HashSet<Edge>> = HashMap::new();
    for pair in pairs {
        // Find new three-way connections
        if let (Some(conn1s), Some(conn2s)) = (
            nminus1way_connections.get(&pair.name1),
            nminus1way_connections.get(&pair.name2),
        ) {
            for shared_edge in conn1s.intersection(conn2s) {
                // If `pair.name1` and `pair.name2` share an edge `shared_edge`, then it means that
                // `pair.name1` and everything inside `shared_edge` is mutually connected, and also
                // `pair.name2` and everything inside `shared_edge` is mutually connected. Since we
                // have now found that `pair.name1` and `pair.name2` are connected, this means that
                // `pair.name1`, `pair.name2`, and everything inside `shared_edge` are mutually
                // connected.
                // We now need to add the edges from pair.name1 to (shared_edge + pair.name2), and
                // the edge from pair.name2 to (shared_edge + pair.name1). (The other permutations
                // don't need to be handled now, because they will be handled when we get to that
                // appropriate pair.)
                let mut all_connected_computers_with_p1 = shared_edge.others.clone();
                all_connected_computers_with_p1.insert(pair.name1.clone());
                edges.entry(pair.name2.clone()).or_default().insert(Edge {
                    others: all_connected_computers_with_p1,
                });
                let mut all_connected_computers_with_p2 = shared_edge.others.clone();
                all_connected_computers_with_p2.insert(pair.name2.clone());
                edges.entry(pair.name1.clone()).or_default().insert(Edge {
                    others: all_connected_computers_with_p2,
                });
            }
        }
    }
    edges
}

fn collapse_connections(nway_connections: HashMap<String, HashSet<Edge>>) -> Vec<BTreeSet<String>> {
    let mut collapsed = Vec::new();
    for (comp, edges) in nway_connections {
        for mut edge in edges {
            edge.others.insert(comp.clone());
            collapsed.push(edge.others);
        }
    }
    collapsed
}

fn has_t(b: &BTreeSet<String>) -> bool {
    b.iter().any(|c| c.starts_with("t"))
}

fn find_largest_connection(pairs: &[Pair]) -> Vec<String> {
    let mut nm1_conns = find_twoway_connections(pairs);
    let mut n_conns = find_nway_connections(pairs, nm1_conns.clone());
    let mut n = 3;
    while !n_conns.is_empty() {
        println!(
            "{}-way connections: {} nodes, {} edges",
            n,
            n_conns.len(),
            n_conns.values().map(|x| x.len()).sum::<usize>()
        );
        nm1_conns = n_conns;
        n_conns = find_nway_connections(pairs, nm1_conns.clone());
        n += 1;
    }
    // Since n_conns is empty, the largest connection is the one in nm1_conns. Note that nm1_conns
    // will have more than 1 entry because it counts permutations rather than combinations, so we
    // can just take the first one (we assume that the puzzle has a unique solution)
    let (k, vs) = nm1_conns.into_iter().next().unwrap();
    let mut v = vs.into_iter().next().unwrap();
    v.others.insert(k);
    v.others.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input(input);
    let twoway_connections = find_twoway_connections(&pairs);
    let threeway_connections = find_nway_connections(&pairs, twoway_connections);
    let collapsed = collapse_connections(threeway_connections);
    let has_t = collapsed.into_iter().filter(has_t).count();
    Some(has_t as u32 / 3) // avoid counting the same connection
}

pub fn part_two(input: &str) -> Option<String> {
    let pairs = parse_input(input);
    let mut largest_connection = find_largest_connection(&pairs);
    largest_connection.sort();
    Some(largest_connection.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
