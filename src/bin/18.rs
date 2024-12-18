advent_of_code::solution!(18);

use itertools::iproduct;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

fn get_adjacents(coord: (usize, usize), max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();
    if coord.0 > 0 {
        adjacents.push((coord.0 - 1, coord.1));
    }
    if coord.0 < max_i {
        adjacents.push((coord.0 + 1, coord.1));
    }
    if coord.1 > 0 {
        adjacents.push((coord.0, coord.1 - 1));
    }
    if coord.1 < max_j {
        adjacents.push((coord.0, coord.1 + 1));
    }
    adjacents
}

// Largely copied from Day 16
struct Graph<T> {
    nodes: Vec<T>,
    // Map of starting node -> (ending node, weight)
    edges: HashMap<T, Vec<(T, u32)>>,
    start_node: T,
    end_node: T,
}

impl<T> Graph<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone,
{
    fn solve(&self) -> u32 {
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
            // If it's the target node, return successfully
            if *n == self.end_node {
                return d;
            }
            // If the remaining nodes are unreachable, break
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
        panic!("Could not reach target node");
    }

    fn can_reach_start_node(&self) -> bool {
        let mut reachable = HashSet::new();
        let mut seen = HashSet::new();
        let mut to_process = vec![self.end_node.clone()];
        while let Some(n) = to_process.pop() {
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n.clone());
            reachable.insert(n.clone());
            for (n2, _) in self.edges[&n].iter() {
                if n2 == &self.start_node {
                    return true;
                }
                if !seen.contains(n2) {
                    to_process.push(n2.clone());
                }
            }
        }
        false
    }
}

// NOTE: This output is transposed relative to the problem description
fn _pretty_print(graph: &Graph<(usize, usize)>, max_i: usize, max_j: usize) {
    for i in 0..=max_i {
        for j in 0..=max_j {
            if graph.nodes.contains(&(i, j)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn parse_input(input: &str, max_i: usize, max_j: usize, n_steps: usize) -> Graph<(usize, usize)> {
    // Collect nodes
    let forbidden_points = input
        .lines()
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>();
            assert_eq!(nums.len(), 2);
            (nums[0], nums[1])
        })
        .take(n_steps)
        .collect::<Vec<_>>();
    let nodes = iproduct!(0..=max_i, 0..=max_j)
        .filter(|point| !forbidden_points.contains(point))
        .collect::<Vec<_>>();

    // Collect edges
    let mut edges = HashMap::new();
    for n in nodes.clone() {
        let allowed_adjacents = get_adjacents(n, max_i, max_j)
            .into_iter()
            .filter(|adj| !forbidden_points.contains(adj))
            .map(|adj| (adj, 1)) // Each edge has a weight of 1
            .collect::<Vec<_>>();
        edges.insert(n, allowed_adjacents);
    }

    Graph {
        nodes,
        edges,
        start_node: (0, 0),
        end_node: (max_i, max_j),
    }
}

fn get_nth_byte(input: &str, n: usize) -> (usize, usize) {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>();
            assert_eq!(nums.len(), 2);
            (nums[0], nums[1])
        })
        .nth(n)
        .expect("Could not find nth byte")
}

pub fn part_one(input: &str) -> Option<u32> {
    // let graph = parse_input(input, 6, 6, 12);
    // _pretty_print(&graph, 6, 6);
    let graph = parse_input(input, 70, 70, 1024);
    Some(graph.solve())
}

pub fn part_two(input: &str) -> Option<String> {
    let max_i = 70;
    let max_j = 70;
    let mut min = 0;
    // Technically the maximum should be the number of lines in the input, which is less than this,
    // but because we're using `.take()` it's fine
    let mut max = (max_i + 1) * (max_j + 1);
    let mut guess = (min + max) / 2;

    // Binary search between the interval [min, max]
    while min <= max {
        guess = (min + max) / 2;
        let graph = parse_input(input, max_i, max_j, guess);
        let can_reach = graph.can_reach_start_node();
        println!(
            "min={} max={} guess={} can_reach={}",
            min, max, guess, can_reach
        );
        if can_reach {
            // Not high enough, because there's still a path
            min = guess + 1;
        } else {
            // Too high
            max = guess - 1;
        }
    }

    // `guess` is the minimum number of bytes to block the path, so if we index the bytes from 0,
    // the first byte that blocks the path is `guess - 1`
    let bytes = get_nth_byte(input, guess - 1);
    let bytes_str = format!("{},{}", bytes.0, bytes.1);
    Some(bytes_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
