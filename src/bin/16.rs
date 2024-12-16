advent_of_code::solution!(16);

use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}
impl Dir {
    fn clockwise(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    fn anticlockwise(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    i: usize,
    j: usize,
}
impl Pos {
    fn new(i: usize, j: usize) -> Self {
        Pos { i, j }
    }
    fn next(&self, d: Dir) -> Self {
        match d {
            Dir::N => Pos::new(self.i - 1, self.j),
            Dir::S => Pos::new(self.i + 1, self.j),
            Dir::E => Pos::new(self.i, self.j + 1),
            Dir::W => Pos::new(self.i, self.j - 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Pos,
    dir: Dir,
}
impl Node {
    fn new(pos: Pos, dir: Dir) -> Self {
        Node { pos, dir }
    }
}

// This represents a truly generic graph of anything.
//
// There are no trait bounds here because apparently you have to repeat the trait bounds on the
// impl block
struct Graph<T> {
    nodes: Vec<T>,
    // Map of starting node -> (ending node, weight)
    edges: HashMap<T, Vec<(T, u32)>>,
    start_node: T,
    end_nodes: Vec<T>,
}

impl<T> Graph<T>
where
    T: std::hash::Hash + PartialEq + Eq + Clone,
{
    // Dijsktra's algorithm
    fn solve(&self) -> (u32, HashSet<T>) {
        // Distances lookup table, with 0 for the starting node and a large number for all others
        let mut distances = HashMap::new();
        // prev_nodes[n] is a vector of Nodes, representing the previous step on the shortest
        // path(s) to the node n
        let mut prev_nodes = HashMap::new();
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
            prev_nodes.insert(node, Vec::new());
        }

        // Remove the node with the smallest distance
        while let Some((n, Reverse(d))) = unvisited.pop() {
            unvisited.remove(&n);
            // If the remaining nodes are unreachable, break
            if d == u32::MAX {
                break;
            }
            // Retrieve the edges that begin at the node of interest
            self.edges[n].iter().for_each(|(n2, weight)| {
                let distance_through_n = d + weight;
                match distance_through_n.cmp(&distances[n2]) {
                    // If the distance to the new node is less than the current minimum
                    Ordering::Less => {
                        // Update the distance, and set the previous node to this one
                        distances.insert(n2, distance_through_n);
                        unvisited.change_priority(n2, Reverse(distance_through_n));
                        prev_nodes.insert(n2, vec![n]);
                    }
                    Ordering::Equal => {
                        // If it's equal, then we need to keep track of it as one of the
                        // possible paths
                        prev_nodes.entry(n2).and_modify(|v| v.push(n));
                    }
                    _ => (),
                }
            });
        }

        // Find the end node with the lowest distance
        let end_node = self
            .end_nodes
            .iter()
            .fold(None, |maybe_acc, idx| match maybe_acc {
                None => Some(idx),
                Some(acc) => {
                    if distances[idx] < distances[&acc] {
                        Some(idx)
                    } else {
                        Some(acc)
                    }
                }
            })
            .unwrap();

        // For part 1, return the distance to the end node with the lowest weight
        let p1_sol = distances[&end_node];
        // For part 2, return all nodes on the shortest paths to the end node
        let p2_sol = {
            let mut nodes_on_path = HashSet::<T>::new();
            nodes_on_path.insert(end_node.clone());
            let mut nodes_to_traverse = vec![end_node];
            while let Some(n) = nodes_to_traverse.pop() {
                // If prev_nodes[n] doesn't exist, means we already checked that square
                if let Some(v) = prev_nodes.remove(n) {
                    v.into_iter().for_each(|prev_node| {
                        nodes_on_path.insert(prev_node.clone());
                        nodes_to_traverse.push(prev_node);
                    });
                }
            }
            nodes_on_path
        };
        (p1_sol, p2_sol)
    }
}

impl From<Grid> for Graph<Node> {
    fn from(grid: Grid) -> Self {
        let nodes = grid.get_all_nodes();
        let mut all_edges = HashMap::new();
        let mut end_nodes = Vec::new();
        for node in nodes.iter() {
            if node.pos == grid.end_pos {
                end_nodes.push(*node);
            }
            let edges = grid.edges(node).into_iter().collect::<Vec<_>>();
            all_edges.insert(*node, edges);
        }

        // println!("{:?} nodes", all_nodes_and_idxs.len());
        // println!("{:?} edges", all_edges.values().map(|v| v.len()).sum::<usize>());

        Graph {
            nodes,
            edges: all_edges,
            start_node: Node::new(grid.start_pos, Dir::E),
            end_nodes,
        }
    }
}

#[derive(Debug)]
struct Grid {
    pos: HashSet<Pos>,
    start_pos: Pos,
    end_pos: Pos,
}

impl Grid {
    // Because our 'moves' consist of turning first BEFORE walking forward, there is no way we can
    // enter a state where our back is facing a wall. We can thus get rid of all such nodes at this
    // stage, so that we don't bother constructing their edges. (This trims almost half the nodes,
    // and two-thirds of the edges)
    // This isn't as much of a time save as it sounds like, because the algorithm will never get
    // into such a state where it tries to use those edges (trying to move from an unreachable node
    // corresponds to the point where the popped distance from the priority queue is u32::MAX), but
    // it does shave a few milliseconds off
    fn get_valid_nodes_from_pos(&self, pos: &Pos) -> Vec<Node> {
        let mut nodes = Vec::new();
        if self.pos.contains(&pos.next(Dir::S)) {
            nodes.push(Node::new(*pos, Dir::N));
        }
        if self.pos.contains(&pos.next(Dir::N)) {
            nodes.push(Node::new(*pos, Dir::S));
        }
        if self.pos.contains(&pos.next(Dir::E)) {
            nodes.push(Node::new(*pos, Dir::W));
        }
        // But for the starting position, we must retain East as a valid node
        if self.pos.contains(&pos.next(Dir::W)) || pos == &self.start_pos {
            nodes.push(Node::new(*pos, Dir::E));
        }
        nodes
    }

    // Get all possible nodes from the list of positions
    fn get_all_nodes(&self) -> Vec<Node> {
        self.pos
            .iter()
            .flat_map(|pos| self.get_valid_nodes_from_pos(pos))
            .collect()
    }

    // Get all possible edges from a node, along with their weights
    fn edges(&self, node: &Node) -> Vec<(Node, u32)> {
        let mut edges = Vec::new();
        // Walk forward
        let next_pos = node.pos.next(node.dir);
        if self.pos.contains(&next_pos) {
            edges.push((Node::new(next_pos, node.dir), 1));
        }
        // Rotate 90 then walk forward
        let next_pos = node.pos.next(node.dir.clockwise());
        if self.pos.contains(&next_pos) {
            edges.push((Node::new(next_pos, node.dir.clockwise()), 1001));
        }
        // Rotate -90 then walk forward
        let next_pos = node.pos.next(node.dir.anticlockwise());
        if self.pos.contains(&next_pos) {
            edges.push((Node::new(next_pos, node.dir.anticlockwise()), 1001));
        }
        edges
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        // Parse input String
        let mut legal_points = HashSet::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let pos = Pos::new(i, j);
                match char {
                    '#' => continue,
                    '.' => {
                        legal_points.insert(pos);
                    }
                    'E' => {
                        legal_points.insert(pos);
                        end = Some(pos);
                    }
                    'S' => {
                        legal_points.insert(pos);
                        start = Some(pos);
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }
        let start_pos = start.expect("No start point found");
        let end_pos = end.expect("No end point found");

        // Get rid of dead ends. We detect dead ends by the fact that they only have one neighbour
        // in the grid. However, we must make sure to exclude the start and end points from this
        loop {
            let mut no_dead_ends_found = true;
            let remaining_points = legal_points.clone();
            for p in remaining_points.iter() {
                if *p == start_pos || *p == end_pos {
                    continue;
                }
                let n_neighbours = [
                    p.next(Dir::N),
                    p.next(Dir::S),
                    p.next(Dir::E),
                    p.next(Dir::W),
                ]
                .into_iter()
                .filter(|p| legal_points.contains(p))
                .count();
                if n_neighbours < 2 {
                    no_dead_ends_found = false;
                    legal_points.remove(p);
                }
            }
            if no_dead_ends_found {
                break;
            }
        }

        Grid {
            pos: legal_points,
            start_pos,
            end_pos,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = Graph::from(Grid::from(input));
    Some(graph.solve().0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let graph = Graph::from(Grid::from(input));
    let nodes = graph.solve().1;
    // Remove duplicate squares
    Some(
        nodes
            .into_iter()
            .map(|n| n.pos)
            .collect::<HashSet<_>>()
            .len() as u32,
    )
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
        assert_eq!(result, Some(45));
    }
}
