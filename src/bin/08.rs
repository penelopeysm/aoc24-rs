advent_of_code::solution!(8);

use itertools::iproduct;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    nodes: HashMap<char, Vec<(i32, i32)>>,
}

// Euclid
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Grid {
    fn in_bounds(&self, index: (i32, i32)) -> bool {
        index.0 >= 0 && index.0 < self.nrows as i32 && index.1 >= 0 && index.1 < self.ncols as i32
    }

    fn count_antinodes(&self) -> u32 {
        let mut antinodes = HashSet::new();
        for char_nodes in self.nodes.values() {
            for (node1, node2) in iproduct!(char_nodes, char_nodes) {
                let (x1, y1) = node1;
                let (x2, y2) = node2;
                if node1 == node2 {
                    continue;
                }
                let dx = x2 - x1;
                let dy = y2 - y1;
                let maybe_antinode = (x2 + dx, y2 + dy);
                if self.in_bounds(maybe_antinode) {
                    antinodes.insert(maybe_antinode);
                }
            }
        }
        antinodes.len() as u32
    }

    fn count_resonant_antinodes(&self) -> u32 {
        let mut antinodes = HashSet::new();
        for char_nodes in self.nodes.values() {
            for (node1, node2) in iproduct!(char_nodes, char_nodes) {
                let (x1, y1) = node1;
                let (x2, y2) = node2;
                if node1 == node2 {
                    continue;
                }
                let dx = x2 - x1;
                let dy = y2 - y1;
                let gcd = gcd(dx, dy).abs(); // absolute value to not mess the direction up
                let min_dx = dx / gcd;
                let min_dy = dy / gcd;
                let mut maybe_antinode = (node1.0, node1.1);
                while self.in_bounds(maybe_antinode) {
                    antinodes.insert(maybe_antinode);
                    maybe_antinode = (maybe_antinode.0 + min_dx, maybe_antinode.1 + min_dy);
                }
            }
        }
        antinodes.len() as u32
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut nrows = 0;
        let mut ncols = 0;
        let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for (i, row) in input.lines().enumerate() {
            nrows = i + 1;
            for (j, char) in row.chars().enumerate() {
                ncols = j + 1;
                if char == '.' {
                    continue;
                } else {
                    nodes
                        .entry(char)
                        .and_modify(|e| e.push((i as i32, j as i32)))
                        .or_insert(vec![(i as i32, j as i32)]);
                }
            }
        }
        Self {
            nrows,
            ncols,
            nodes,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(Grid::from(input).count_antinodes())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Grid::from(input).count_resonant_antinodes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
