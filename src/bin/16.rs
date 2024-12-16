advent_of_code::solution!(16);

use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::collections::HashSet;

enum Part {
    One,
    Two,
}

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

#[derive(Debug)]
struct Grid {
    pos: HashSet<Pos>,
    start_pos: Pos,
    end_pos: Pos,
}

type NodeIndex = usize;

struct Graph {
    // Map of starting node -> (ending node, weight)
    edges: HashMap<NodeIndex, Vec<(NodeIndex, u32)>>,
    start_idx: NodeIndex,
    end_idxs: Vec<NodeIndex>,
    n_nodes: usize,
    // We need to keep track of which nodes belong to the same square, because we need to
    // remove duplicate squares later when finding the path
    idxs_to_pos: HashMap<NodeIndex, Pos>,
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
    fn all_nodes_and_idxs(&self) -> (HashMap<Node, NodeIndex>, HashMap<NodeIndex, Pos>) {
        let mut m = HashMap::new();
        let mut n = HashMap::new();
        let mut counter = 0;
        self.pos
            .iter()
            .flat_map(|pos| self.get_valid_nodes_from_pos(pos))
            .for_each(|node| {
                m.insert(node, counter);
                n.insert(counter, node.pos);
                counter += 1;
            });
        (m, n)
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

    fn to_graph(&self) -> Graph {
        let (all_nodes_and_idxs, all_idxs_and_poses) = self.all_nodes_and_idxs();
        let mut all_edges = HashMap::new();
        let mut start_idx = None;
        let mut end_idxs = Vec::new();

        for (node, idx) in all_nodes_and_idxs.iter() {
            if node.pos == self.start_pos && node.dir == Dir::E {
                start_idx = Some(idx);
            }
            if node.pos == self.end_pos {
                end_idxs.push(*idx);
            }
            // Collect edges
            let edges = self
                .edges(node)
                .into_iter()
                .map(|(next_node, weight)| (all_nodes_and_idxs[&next_node], weight))
                .collect::<Vec<_>>();
            all_edges.insert(*idx, edges);
        }

        // println!("{:?} nodes", all_nodes_and_idxs.len());
        // println!("{:?} edges", all_edges.values().map(|v| v.len()).sum::<usize>());

        Graph {
            edges: all_edges,
            start_idx: *start_idx.expect("No start node found"),
            end_idxs,
            n_nodes: all_nodes_and_idxs.len(),
            idxs_to_pos: all_idxs_and_poses,
        }
    }

    // Dijsktra's algorithm
    fn solve(&self, part: Part) -> u32 {
        let graph = self.to_graph();
        // Populate distances table, with 0 for the starting node and a large number for all others
        let mut distances = vec![u32::MAX; graph.n_nodes];
        distances[graph.start_idx] = 0;
        // prev_nodes[idx] is a vector of NodeIndex's, representing the previous step on the
        // shortest path(s) to the node with index idx
        let mut prev_nodes = vec![Vec::<NodeIndex>::new(); graph.n_nodes];

        let mut unvisited = PriorityQueue::new();
        for (i, dist) in distances.iter().enumerate() {
            unvisited.push(i, Reverse(*dist));
        }

        // Remove the node with the smallest distance
        while let Some((n, Reverse(d))) = unvisited.pop() {
            unvisited.remove(&n);
            // If the remaining nodes are unreachable, break
            if d == u32::MAX {
                break;
            }
            // Retrieve the edges that begin at the node of interest
            graph.edges[&n].iter().for_each(|(n2, weight)| {
                let distance_through_n = d + weight;
                match distance_through_n.cmp(&distances[*n2]) {
                    // If the distance to the new node is less than the current minimum
                    Ordering::Less => {
                        // Update the distance, and set the previous node to this one
                        distances[*n2] = distance_through_n;
                        unvisited.change_priority(n2, Reverse(distance_through_n));
                        prev_nodes[*n2] = vec![n];
                    }
                    Ordering::Equal => {
                        // If it's equal, then we need to keep track of it as one of the
                        // possible paths
                        prev_nodes[*n2].push(n);
                    }
                    _ => (),
                }
            });
        }

        // Find the end node with the lowest distance
        let end_node = graph.end_idxs.iter().fold(graph.end_idxs[0], |acc, idx| {
            if distances[*idx] < distances[acc] {
                *idx
            } else {
                acc
            }
        });

        match part {
            // For part 1, return the distance to the end node with the lowest weight
            Part::One => distances[end_node],
            Part::Two => {
                // For part 2, we need to find the squares on the reverse path. This is a bit
                // tricky because our path consists of nodes (which contain position + direction),
                // not just positions. So if we don't remove duplicates, we'll end up
                // double-counting squares that are visited twice via different directions.
                let mut positions_on_path = HashSet::<Pos>::new();
                positions_on_path.insert(graph.idxs_to_pos[&end_node]);
                let mut nodes_to_traverse = vec![end_node];
                while let Some(sq) = nodes_to_traverse.pop() {
                    prev_nodes[sq].clone().into_iter().for_each(|prev_node| {
                        positions_on_path.insert(graph.idxs_to_pos[&prev_node]);
                        nodes_to_traverse.push(prev_node);
                    });
                }
                positions_on_path.len() as u32
            }
        }
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
    let grid = Grid::from(input);
    Some(grid.solve(Part::One))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    Some(grid.solve(Part::Two))
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
