advent_of_code::solution!(10);

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point<T: IntoIterator<Item = (u32, u32)> + FromIterator<(u32, u32)>> {
    row: u32,
    col: u32,
    reachable_nines: T,
}

fn is_adjacent<T: IntoIterator<Item = (u32, u32)> + FromIterator<(u32, u32)>>(
    p1: &Point<T>,
    p2: &Point<T>,
) -> bool {
    p1.row.abs_diff(p2.row) == 1 && p1.col.abs_diff(p2.col) == 0
        || (p1.row.abs_diff(p2.row) == 0 && p1.col.abs_diff(p2.col) == 1)
}

fn sum_trailheads<T: Clone + IntoIterator<Item = (u32, u32)> + FromIterator<(u32, u32)>>(
    mut map: BTreeMap<u32, Vec<Point<T>>>,
) -> u32 {
    // For each value of N < 9, collect all the reachable nines of its neighbours that are N+1
    for n in (0..9).rev() {
        let np1_points = map.remove(&(n + 1)).unwrap();
        let n_points = map.get_mut(&n).unwrap();
        for point in n_points {
            point.reachable_nines = np1_points
                .iter()
                .filter(|p| is_adjacent(point, p))
                .flat_map(|p| p.reachable_nines.clone())
                .collect();
        }
    }
    map.remove(&0).unwrap().into_iter().fold(0, |acc, p| {
        // .len() isn't provided by a trait, so .into_iter().count() is used instead
        acc + p.reachable_nines.into_iter().count() as u32
    })
}

fn parse_input<T: Default + IntoIterator<Item = (u32, u32)> + FromIterator<(u32, u32)>>(
    input: &str,
) -> BTreeMap<u32, Vec<Point<T>>> {
    let mut map: BTreeMap<u32, Vec<Point<T>>> = BTreeMap::new();
    for i in 0..10 {
        map.insert(i, Vec::new());
    }
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char.to_digit(10) {
                None => panic!("Invalid input"),
                Some(n) => {
                    let point = Point {
                        row: i as u32,
                        col: j as u32,
                        reachable_nines: if n == 9 {
                            [(i as u32, j as u32)].into_iter().collect()
                        } else {
                            T::default()
                        },
                    };
                    map.get_mut(&n).unwrap().push(point);
                }
            }
        }
    }
    map
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_trailheads(parse_input::<BTreeSet<_>>(input)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_trailheads(parse_input::<Vec<_>>(input)))
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
