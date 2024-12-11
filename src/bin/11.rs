advent_of_code::solution!(11);

use std::collections::BTreeMap;

fn stone(x: u64) -> (u64, Option<u64>) {
    if x == 0 {
        (1, None)
    } else {
        let mut temp = x;
        let mut n_digits = 0;
        while temp >= 1 {
            temp /= 10;
            n_digits += 1;
        }
        if n_digits % 2 == 0 {
            let modulus = 10u64.pow(n_digits / 2);
            (x / modulus, Some(x % modulus))
        } else {
            (x * 2024, None)
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

// Return the number of stones after n blinks, starting from a single stone with value x
fn get_len_after_n_blinks(x: u64, n: u8, storage: &mut BTreeMap<(u64, u8), u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    match storage.get(&(x, n)) {
        // Already calculated
        Some(&x) => x,
        // Not yet calculated
        None => {
            // Blink once then recurse
            let result: u64;
            let (a, maybe_b) = stone(x);
            match maybe_b {
                Some(b) => {
                    storage.insert((x, 1), 2);
                    result = get_len_after_n_blinks(a, n - 1, storage)
                        + get_len_after_n_blinks(b, n - 1, storage);
                    storage.insert((x, n), result);
                }
                None => {
                    storage.insert((x, 1), 1);
                    result = get_len_after_n_blinks(a, n - 1, storage);
                    storage.insert((x, n), result);
                }
            }
            result
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let xs = parse_input(input);
    let mut storage: BTreeMap<(u64, u8), u64> = BTreeMap::new();
    Some(xs.into_iter().fold(0, |acc, x| {
        acc + get_len_after_n_blinks(x, 25, &mut storage)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let xs = parse_input(input);
    let mut storage: BTreeMap<(u64, u8), u64> = BTreeMap::new();
    Some(xs.into_iter().fold(0, |acc, x| {
        acc + get_len_after_n_blinks(x, 75, &mut storage)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // Puzzle answer isn't given
        assert!(result.is_some());
    }
}
