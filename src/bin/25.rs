advent_of_code::solution!(25);

use itertools::iproduct;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Lock {
    lengths: Vec<u32>,
}
impl From<&str> for Lock {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let ncols = lines[0].len();
        let mut lengths = std::iter::repeat_n(0, ncols).collect::<Vec<u32>>();
        for line in lines[1..].iter() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    lengths[i] += 1;
                }
            }
        }
        Lock { lengths }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Key {
    lengths: Vec<u32>,
}
impl From<&str> for Key {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let ncols = lines[0].len();
        let mut lengths = std::iter::repeat_n(0, ncols).collect::<Vec<u32>>();
        for line in lines[0..lines.len() - 1].iter() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    lengths[i] += 1;
                }
            }
        }
        Key { lengths }
    }
}

fn fit(lock: &Lock, key: &Key) -> bool {
    for (l, k) in lock.lengths.iter().zip(key.lengths.iter()) {
        if l + k > 5 {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> (HashSet<Lock>, HashSet<Key>) {
    let mut locks = HashSet::new();
    let mut keys = HashSet::new();
    for block in input.split("\n\n") {
        let first_char = block.chars().next().unwrap();
        if first_char == '#' {
            locks.insert(Lock::from(block));
        } else if first_char == '.' {
            keys.insert(Key::from(block));
        }
    }
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);
    Some(
        iproduct!(locks.iter(), keys.iter())
            .filter(|(lock, key)| fit(lock, key))
            .count() as u32,
    )
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
