advent_of_code::solution!(20);

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet};

// Largely copied from Day 18
#[derive(Clone, Debug)]
struct Graph<T> {
    nodes: HashSet<T>,
    // Map of starting node -> (ending node, weight)
    edges: HashMap<T, Vec<(T, u32)>>,
    start_node: T,
    end_node: T,
}

impl<T> Graph<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone,
{
    // Create a new graph with the start and end nodes reversed
    fn reverse_start_and_end(&self) -> Graph<T> {
        let mut new_graph = self.clone();
        std::mem::swap(&mut new_graph.start_node, &mut new_graph.end_node);
        new_graph
    }

    // Returns the minimum distance from the start node to every node
    fn solve(&self) -> HashMap<&T, u32> {
        // Distances lookup table, with 0 for the starting node and a large number for all others
        let mut distances = HashMap::new();
        // Priority queue to keep track of the unvisited nodes
        let mut unvisited = PriorityQueue::new();

        // Initialise the collections
        for node in self.nodes.iter() {
            let dist = if node == &self.start_node {
                0
            } else {
                u32::MAX
            };
            distances.insert(node, dist);
            unvisited.push(node, Reverse(dist));
        }

        // Remove the node with the smallest distance
        while let Some((n, Reverse(d))) = unvisited.pop() {
            // If the remaining nodes are unreachable, finish
            if d == u32::MAX {
                break;
            }
            // Retrieve the edges that begin at the node of interest
            self.edges[n].iter().for_each(|(n2, weight)| {
                let distance_through_n = d + weight;
                if distance_through_n < distances[n2] {
                    distances.insert(n2, distance_through_n);
                    unvisited.change_priority(n2, Reverse(distance_through_n));
                }
            });
        }
        distances
    }
}

impl Graph<Point> {
    fn get_cheats_with_manhattan<F: Fn(u32) -> bool>(
        self,
        manhattan_filter: F,
    ) -> BTreeMap<u32, u32> {
        let rgraph = self.reverse_start_and_end();
        let fgraph = self;

        let fdists = fgraph.solve();
        let rdists = rgraph.solve();

        let original_solution = fdists.get(&fgraph.end_node).unwrap();

        // Construct mapping of (time saved, number of choices)
        let mut cheats = BTreeMap::<u32, u32>::new();
        // Iterate over the points in the forward graph.
        for (fnode, fdist) in fdists.iter() {
            // Skip unreachable nodes
            if fdist == &u32::MAX {
                continue;
            }
            // Get all points in graph
            for (rnode, rdist) in rdists.iter() {
                // Skip unreachable nodes
                if rdist == &u32::MAX {
                    continue;
                }
                let manhattan = fnode.manhattan(rnode) as u32;
                if manhattan_filter(manhattan) {
                    // Otherwise, calculate the time saved
                    let cheating_time = fdist + rdist + manhattan;
                    if cheating_time < *original_solution {
                        let timesave = original_solution - cheating_time;
                        cheats.entry(timesave).and_modify(|e| *e += 1).or_insert(1);
                    }
                }
            }
        }
        cheats
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Point { i, j }
    }

    fn get_adjacents(&self) -> HashSet<Point> {
        HashSet::from([
            Point::new(self.i + 1, self.j),
            Point::new(self.i - 1, self.j),
            Point::new(self.i, self.j + 1),
            Point::new(self.i, self.j - 1),
        ])
    }

    fn manhattan(&self, other: &Point) -> usize {
        self.i.abs_diff(other.i) + self.j.abs_diff(other.j)
    }
}

// Input parsing
impl From<&str> for Graph<Point> {
    fn from(input: &str) -> Self {
        // Parse nodes from the input string
        let mut nodes = HashSet::new();
        let mut maybe_start_node = None;
        let mut maybe_end_node = None;
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let pos = Point::new(i, j);
                match char {
                    '#' => continue,
                    '.' => {
                        nodes.insert(pos);
                    }
                    'E' => {
                        maybe_end_node = Some(pos.clone());
                        nodes.insert(pos);
                    }
                    'S' => {
                        maybe_start_node = Some(pos.clone());
                        nodes.insert(pos);
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }
        let start_node = maybe_start_node.expect("No start point found");
        let end_node = maybe_end_node.expect("No end point found");

        // Construct edges
        let mut edges = HashMap::new();
        for node in &nodes {
            let mut this_adjacents = Vec::new();
            for adjacents in node.get_adjacents() {
                if nodes.contains(&adjacents) {
                    this_adjacents.push((adjacents, 1));
                }
            }
            edges.insert(node.clone(), this_adjacents);
        }

        Graph {
            nodes,
            edges,
            start_node,
            end_node,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = Graph::from(input);
    let cheats = graph.get_cheats_with_manhattan(|m| m == 2);
    Some(
        cheats
            .iter()
            .filter(|(time, _)| **time >= 100)
            .map(|(_, choices)| choices)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let graph = Graph::from(input);
    let cheats = graph.get_cheats_with_manhattan(|m| m <= 20);
    Some(
        cheats
            .iter()
            .filter(|(time, _)| **time >= 100)
            .map(|(_, choices)| choices)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
