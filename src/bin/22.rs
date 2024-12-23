advent_of_code::solution!(22);

use std::collections::{HashMap, HashSet};

fn next_secret(secret: u64) -> u64 {
    let a = secret * 64; // 2^6
    let secret = secret ^ a;
    let secret = secret % 16777216; // 2^24

    let b = secret / 32;
    let secret = secret ^ b;
    let secret = secret % 16777216;

    let c = secret * 2048; // 2^11
    let secret = secret ^ c;
    secret % 16777216
}

fn nth_secret_after(first: u64, n: u32) -> u64 {
    let mut secret = first;
    for _ in 0..n {
        secret = next_secret(secret);
    }
    secret
}

fn get_possible_prices(first: u64, n: u32) -> HashMap<Vec<i64>, u64> {
    let mut secret = first;
    let mut prices = HashMap::new();
    let mut changes = Vec::new();
    let mut last_price = first % 10;
    for _ in 0..n {
        secret = next_secret(secret);
        let this_price = secret % 10;
        let price_diff = (this_price as i64) - (last_price as i64);
        // Tabulate the changes
        if changes.len() == 4 {
            changes.remove(0);
        }
        changes.push(price_diff);
        // Add them to the hashmap
        if changes.len() == 4 {
            prices.entry(changes.clone()).or_insert(this_price);
        }
        // Update the last price
        last_price = this_price;
    }
    prices
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse_input(input);
    let results = inputs
        .into_iter()
        .map(|first| nth_secret_after(first, 2000))
        .sum();
    Some(results)
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse_input(input);
    let mut merged = HashMap::new(); // map of changes -> total price
    let mut largest_val = 0;
    inputs
        .into_iter()
        .map(|first| get_possible_prices(first, 2000))
        .for_each(|prices| {
            for (key, val) in prices.into_iter() {
                let new_val = merged.entry(key).and_modify(|v| *v += val).or_insert(val);
                largest_val = largest_val.max(*new_val);
            }
        });
    Some(largest_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // NOTE: This is the solution for (1, 2, 3, 2024).
        // The original example in part 1 of the problem is (1, 10, 100, 2024) which
        // gives a result of 37327623
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
