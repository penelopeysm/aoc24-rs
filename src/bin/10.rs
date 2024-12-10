advent_of_code::solution!(10);

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    row: u32,
    col: u32,
    reachable_nines: BTreeSet<(u32, u32)>,
    score: u32,
}

fn is_adjacent(p1: &Point, p2: &Point) -> bool {
    p1.row.abs_diff(p2.row) == 1 && p1.col.abs_diff(p2.col) == 0
        || (p1.row.abs_diff(p2.row) == 0 && p1.col.abs_diff(p2.col) == 1)
}

fn sum_trailheads(mut map: BTreeMap<u32, Vec<Point>>) -> (u32, u32) {
    // For each value of N < 9, sum the scores of all adjacent N+1's
    for n in (0..9).rev() {
        let np1_points = map.remove(&(n + 1)).unwrap();
        let n_points = map.get_mut(&n).unwrap();
        for point in n_points {
            point.reachable_nines = np1_points
                .iter()
                .filter(|p| is_adjacent(point, p))
                .flat_map(|p| p.reachable_nines.clone())
                .collect();
            point.score = np1_points
                .iter()
                .filter(|p| is_adjacent(point, p))
                .fold(0, |acc, p| acc + p.score);
        }
    }
    // Return part 1 and part 2 together
    let p1_sol = map.get(&0)
        .unwrap()
        .iter()
        .fold(0, |acc, p| acc + p.reachable_nines.len() as u32);
    let p2_sol = map.get(&0)
        .unwrap()
        .iter()
        .fold(0, |acc, p| acc + p.score);
    (p1_sol, p2_sol)
}

fn parse_input(input: &str) -> BTreeMap<u32, Vec<Point>> {
    let mut map: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
    for i in 0..10 {
        map.insert(i, Vec::new());
    }
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                None => panic!("Invalid input"),
                Some(n) => {
                    let mut point = Point {
                        row: i as u32,
                        col: j as u32,
                        reachable_nines: BTreeSet::new(),
                        score: if n == 9 { 1 } else { 0 },
                    };
                    if n == 9 {
                        point.reachable_nines.insert((i as u32, j as u32));
                    }
                    map.get_mut(&n).unwrap().push(point);
                }
            }
        }
    }
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_trailheads(parse_input(input)).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_trailheads(parse_input(input)).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
