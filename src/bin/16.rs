advent_of_code::solution!(16);

use std::cmp::Ordering;
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

// // Get the cost of travelling from n1 to n2
// fn _get_edge_weight(n1: &Node, n2: &Node) -> Option<u32> {
//     // Rotating
//     if n1.i == n2.i && n1.j == n2.j {
//         match (n1.dir, n2.dir) {
//             (Dir::N, Dir::E) => Some(1000),
//             (Dir::N, Dir::W) => Some(1000),
//             (Dir::E, Dir::N) => Some(1000),
//             (Dir::E, Dir::S) => Some(1000),
//             (Dir::S, Dir::E) => Some(1000),
//             (Dir::S, Dir::W) => Some(1000),
//             (Dir::W, Dir::N) => Some(1000),
//             (Dir::W, Dir::S) => Some(1000),
//             _ => None,
//         }
//     }
//     // Traavelling
//     else if (n1.i == n2.i && n1.j + 1 == n2.j && n1.dir == n2.dir && n1.dir == Dir::E)
//         || (n1.i == n2.i && n1.j == n2.j + 1 && n1.dir == n2.dir && n1.dir == Dir::W)
//         || (n1.i + 1 == n2.i && n1.j == n2.j && n1.dir == n2.dir && n1.dir == Dir::S)
//         || (n1.i == n2.i + 1 && n1.j == n2.j && n1.dir == n2.dir && n1.dir == Dir::N)
//     {
//         Some(1)
//     } else {
//         None
//     }
// }

// // Same but about 3x faster
// fn get_edge_weight2(n1: &Node, n2: &Node) -> Option<u32> {
//     if n1.i == n2.i && n1.j + 1 == n2.j {
//         match (n1.dir, n2.dir) {
//             (Dir::E, Dir::E) => Some(1),
//             (Dir::N, Dir::E) => Some(1001),
//             (Dir::S, Dir::E) => Some(1001),
//             _ => None,
//         }
//     } else if n1.i == n2.i && n1.j == n2.j + 1 {
//         match (n1.dir, n2.dir) {
//             (Dir::W, Dir::W) => Some(1),
//             (Dir::N, Dir::W) => Some(1001),
//             (Dir::S, Dir::W) => Some(1001),
//             _ => None,
//         }
//     } else if n1.i + 1 == n2.i && n1.j == n2.j {
//         match (n1.dir, n2.dir) {
//             (Dir::S, Dir::S) => Some(1),
//             (Dir::E, Dir::S) => Some(1001),
//             (Dir::W, Dir::S) => Some(1001),
//             _ => None,
//         }
//     } else if n1.i == n2.i + 1 && n1.j == n2.j {
//         match (n1.dir, n2.dir) {
//             (Dir::N, Dir::N) => Some(1),
//             (Dir::E, Dir::N) => Some(1001),
//             (Dir::W, Dir::N) => Some(1001),
//             _ => None,
//         }
//     } else {
//         None
//     }
// }

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
    fn new(i: usize, j: usize, dir: Dir) -> Self {
        Node {
            pos: Pos::new(i, j),
            dir,
        }
    }
    fn new_pos(pos: Pos, dir: Dir) -> Self {
        Node { pos, dir }
    }
    fn edges(&self, grid: &Grid) -> Vec<(Node, u32)> {
        let mut edges = Vec::new();
        let next_pos = self.pos.next(self.dir);
        if grid.pos.contains(&next_pos) {
            edges.push((Node::new_pos(next_pos, self.dir), 1));
        }
        let next_pos = self.pos.next(self.dir.clockwise());
        if grid.pos.contains(&next_pos) {
            edges.push((Node::new_pos(next_pos, self.dir.clockwise()), 1001));
        }
        let next_pos = self.pos.next(self.dir.anticlockwise());
        if grid.pos.contains(&next_pos) {
            edges.push((Node::new_pos(next_pos, self.dir.anticlockwise()), 1001));
        }
        edges
    }
}

#[derive(Debug)]
struct Grid {
    pos: Vec<Pos>,
    start_pos: Pos,
    end_pos: Pos,
}

type NodeIndex = usize;

struct Graph {
    edges: Vec<(NodeIndex, NodeIndex, u32)>,
    start_idx: NodeIndex,
    end_idxs: Vec<NodeIndex>,
    n_nodes: usize,
    idxs_to_pos: HashMap<NodeIndex, Pos>,
}

impl Grid {
    // Get all possible nodes
    fn all_nodes_and_idxs(&self) -> (HashMap<Node, NodeIndex>, HashMap<NodeIndex, Pos>) {
        let mut m = HashMap::new();
        let mut n = HashMap::new();
        let mut counter = 0;
        self.pos
            .iter()
            .flat_map(|pos| {
                vec![
                    Node::new_pos(*pos, Dir::N),
                    Node::new_pos(*pos, Dir::S),
                    Node::new_pos(*pos, Dir::E),
                    Node::new_pos(*pos, Dir::W),
                ]
            })
            .for_each(|node| {
                m.insert(node, counter);
                n.insert(counter, node.pos);
                counter += 1;
            });
        (m, n)
    }

    fn to_graph(&self) -> Graph {
        let (all_nodes_and_idxs, all_idxs_and_poses) = self.all_nodes_and_idxs();
        let mut all_edges = Vec::new();
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
            for (next_node, weight) in node.edges(self) {
                match all_nodes_and_idxs.get(&next_node) {
                    Some(next_idx) => {
                        all_edges.push((*idx, *next_idx, weight));
                    }
                    None => panic!("Shouldn't happen"),
                }
            }
        }

        Graph {
            edges: all_edges,
            start_idx: *start_idx.expect("No start node found"),
            end_idxs,
            n_nodes: all_nodes_and_idxs.len(),
            idxs_to_pos: all_idxs_and_poses,
        }
    }

    fn solve(&self, part: Part) -> u32 {
        // Populate distances table, with 0 for the starting node and a large number for all others
        let graph = self.to_graph();
        let mut distances = vec![u32::MAX; graph.n_nodes];
        let mut prev_nodes = vec![Vec::<NodeIndex>::new(); graph.n_nodes];
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
            // Remove that node
            unvisited.remove(&n);
            // If the remaining nodes are unreachable, break
            if distances[n] == u32::MAX {
                break;
            }
            // Extract the edges that touch the node of interest
            graph.edges.iter().for_each(|(n1, n2, weight)| {
                if *n1 == n {
                    let distance_through_n = distances[n] + weight;
                    match distance_through_n.cmp(&distances[*n2]) {
                        Ordering::Less => {
                            distances[*n2] = distance_through_n;
                            prev_nodes[*n2] = vec![n];
                        }
                        Ordering::Equal => {
                            prev_nodes[*n2].push(n);
                        }
                        _ => (),
                    }
                }
            });
        }

        // Find the end node with the lowest weight
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
                // Find the nodes on the reverse path
                let mut path = HashSet::new();
                path.insert(end_node);
                let mut nodes_to_traverse = vec![end_node];
                while let Some(sq) = nodes_to_traverse.pop() {
                    path.extend(prev_nodes[sq].clone());
                    nodes_to_traverse.extend(prev_nodes[sq].clone());
                }
                // Need to deduplicate nodes on the same square
                let mut poses_on_path = HashSet::new();
                path.iter().for_each(|node_idx| {
                    let pos = graph.idxs_to_pos.get(node_idx).expect("Node not found");
                    poses_on_path.insert(pos);
                });
                poses_on_path.len() as u32
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut legal_points = Vec::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let pos = Pos::new(i, j);
                match char {
                    '#' => continue,
                    '.' => legal_points.push(pos),
                    'E' => {
                        legal_points.push(pos);
                        end = Some(pos);
                    }
                    'S' => {
                        legal_points.push(pos);
                        start = Some(pos);
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
        }
        Grid {
            pos: legal_points,
            start_pos: start.expect("No start point found"),
            end_pos: end.expect("No end point found"),
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
