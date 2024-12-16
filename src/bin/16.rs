advent_of_code::solution!(16);

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    i: usize,
    j: usize,
    dir: Dir,
}

// Get the cost of travelling from n1 to n2
fn _get_edge_weight(n1: &Node, n2: &Node) -> Option<u32> {
    // Rotating
    if n1.i == n2.i && n1.j == n2.j {
        match (n1.dir, n2.dir) {
            (Dir::N, Dir::E) => Some(1000),
            (Dir::N, Dir::W) => Some(1000),
            (Dir::E, Dir::N) => Some(1000),
            (Dir::E, Dir::S) => Some(1000),
            (Dir::S, Dir::E) => Some(1000),
            (Dir::S, Dir::W) => Some(1000),
            (Dir::W, Dir::N) => Some(1000),
            (Dir::W, Dir::S) => Some(1000),
            _ => None,
        }
    }
    // Traavelling
    else if (n1.i == n2.i && n1.j + 1 == n2.j && n1.dir == n2.dir && n1.dir == Dir::E)
        || (n1.i == n2.i && n1.j == n2.j + 1 && n1.dir == n2.dir && n1.dir == Dir::W)
        || (n1.i + 1 == n2.i && n1.j == n2.j && n1.dir == n2.dir && n1.dir == Dir::S)
        || (n1.i == n2.i + 1 && n1.j == n2.j && n1.dir == n2.dir && n1.dir == Dir::N)
    {
        Some(1)
    } else {
        None
    }
}

// Same but about 3x faster
fn get_edge_weight2(n1: &Node, n2: &Node) -> Option<u32> {
    if n1.i == n2.i && n1.j + 1 == n2.j {
        match (n1.dir, n2.dir) {
            (Dir::E, Dir::E) => Some(1),
            (Dir::N, Dir::E) => Some(1001),
            (Dir::S, Dir::E) => Some(1001),
            _ => None,
        }
    } else if n1.i == n2.i && n1.j == n2.j + 1 {
        match (n1.dir, n2.dir) {
            (Dir::W, Dir::W) => Some(1),
            (Dir::N, Dir::W) => Some(1001),
            (Dir::S, Dir::W) => Some(1001),
            _ => None,
        }
    } else if n1.i + 1 == n2.i && n1.j == n2.j {
        match (n1.dir, n2.dir) {
            (Dir::S, Dir::S) => Some(1),
            (Dir::E, Dir::S) => Some(1001),
            (Dir::W, Dir::S) => Some(1001),
            _ => None,
        }
    } else if n1.i == n2.i + 1 && n1.j == n2.j {
        match (n1.dir, n2.dir) {
            (Dir::N, Dir::N) => Some(1),
            (Dir::E, Dir::N) => Some(1001),
            (Dir::W, Dir::N) => Some(1001),
            _ => None,
        }
    } else {
        None
    }
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Node>,
    start_node: Node,
    end: (usize, usize),
}

struct Graph {
    edges: Vec<(usize, usize, u32)>,
    start_idx: usize,
    end_idxs: Vec<usize>,
    n_nodes: usize,
}

impl Grid {
    // Returns a graph containing:
    //   - The edges of the graph: each edge is itself a 3-tuple of (node1, node2, weight)
    //   - The starting node
    //   - The possible ending nodes
    //   - The number of nodes in the graph
    // Note that nodes are represented by indices rather than their positions, because
    // after this function there is no need to know their positions any more
    fn to_graph(&self) -> Graph {
        let mut edges = Vec::new();
        let mut counter = 0;
        let mut nodes_lookup = HashMap::new();
        let mut start_idx = None;
        let mut end_idxs = Vec::new();
        for (idx1, node_i) in self.nodes.iter().enumerate() {
            if node_i == &self.start_node {
                start_idx = Some(idx1);
            }
            if (node_i.i, node_i.j) == self.end {
                end_idxs.push(idx1);
            }
            for (idx2, node_j) in self.nodes.iter().enumerate() {
                if let Some(weight) = get_edge_weight2(node_i, node_j) {
                    edges.push((idx1, idx2, weight));
                    if let std::collections::hash_map::Entry::Vacant(e) = nodes_lookup.entry(idx1) {
                        e.insert(counter);
                        counter += 1;
                    }
                    if let std::collections::hash_map::Entry::Vacant(e) = nodes_lookup.entry(idx2) {
                        e.insert(counter);
                        counter += 1;
                    }
                }
            }
        }
        // Reindex to strip out all nodes that can't be reached
        let edges = edges
            .into_iter()
            .map(|(n1, n2, w)| (nodes_lookup[&n1], nodes_lookup[&n2], w))
            .collect();
        let start_idx = nodes_lookup[&start_idx.expect("Start node not found")];
        let end_idxs = end_idxs.into_iter().map(|idx| nodes_lookup[&idx]).collect();
        Graph {
            edges,
            start_idx,
            end_idxs,
            n_nodes: nodes_lookup.len(),
        }
    }

    fn solve(&self) -> u32 {
        // Populate distances table, with 0 for the starting node and a large number for all others
        let graph = self.to_graph();
        let mut distances = vec![u32::MAX; graph.n_nodes];
        distances[graph.start_idx] = 0;

        let mut unvisited = (0..graph.n_nodes).collect::<HashSet<usize>>();
        while !unvisited.is_empty() {
            // Get a node of interest
            let init = *unvisited
                .iter()
                .next()
                .expect("Shouldn't happen, we checked the set was not empty");
            let n = unvisited.iter().fold(init, |acc, idx| {
                if distances[*idx] < distances[acc] {
                    *idx
                } else {
                    acc
                }
            });
            if distances[n] == u32::MAX {
                break;
            }
            // Extract the edges that touch the node of interest
            let edges = graph
                .edges
                .clone()
                .into_iter()
                .filter(|e| e.0 == n)
                .collect::<Vec<_>>();
            // Fill in any weights
            edges.into_iter().for_each(|(_, n2, weight)| {
                distances[n2] = distances[n2].min(distances[n] + weight);
            });
            // Remove that node
            unvisited.remove(&n);
            // If we've already visited all the target nodes, break
            if graph.end_idxs.iter().all(|idx| !unvisited.contains(idx)) {
                break;
            }
        }

        // Then pick the end node with the lowest weight
        graph
            .end_idxs
            .into_iter()
            .fold(u32::MAX, |acc, idx| acc.min(distances[idx]))
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut legal_points = Vec::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                match char {
                    '#' => continue,
                    '.' => legal_points.push((i, j)),
                    'E' => {
                        legal_points.push((i, j));
                        end = Some((i, j));
                    }
                    'S' => {
                        legal_points.push((i, j));
                        start = Some(Node { i, j, dir: Dir::E });
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }
        let nodes = legal_points
            .iter()
            .flat_map(|(i, j)| {
                vec![
                    Node {
                        i: *i,
                        j: *j,
                        dir: Dir::N,
                    },
                    Node {
                        i: *i,
                        j: *j,
                        dir: Dir::S,
                    },
                    Node {
                        i: *i,
                        j: *j,
                        dir: Dir::E,
                    },
                    Node {
                        i: *i,
                        j: *j,
                        dir: Dir::W,
                    },
                ]
            })
            .collect();
        Grid {
            nodes,
            start_node: start.expect("No start point found"),
            end: end.expect("No end point found"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    Some(grid.solve())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
