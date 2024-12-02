advent_of_code::solution!(2);

/// Parse whitespace-separated integers into a vector
fn parse_line(line: &str) -> Vec<i32> { line.split_whitespace()
        .map(|n| n.parse().expect("Not an integer"))
        .collect()
}

/// Given a list of numbers a_0, a_1, ..., a_(n-1), calculate the differences d_i where d_i =
/// a_{i+1} - a_i for i = 0, 1, ..., n-2.
fn calculate_diffs(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..(numbers.len() - 1) {
        result.push(numbers[i + 1] - numbers[i]);
    }
    result
}

/// Check if a series of differences is fully safe
fn diffs_are_safe(diffs: &[i32]) -> bool {
    // Ascending
    if diffs.iter().all(|d| (1..4).contains(d)) {
        return true;
    }
    // Descending
    if diffs.iter().all(|d| (-3..0).contains(d)) {
        return true;
    }
    false
}

/// Check if a series of numbers is fully safe
fn is_safe(numbers: &[i32]) -> bool {
    diffs_are_safe(&calculate_diffs(numbers))
}

/// 'Contract' a list of differences. Effectively, passing idx=j will remove the (j+1)-th number of
/// the original list and recalculate the diffs, except that it doesn't use the original list.
fn contract_diffs(diffs: &[i32], idx: usize) -> Vec<i32> {
    // Clone all entries to Somes
    let mut new_diffs = diffs.iter().map(|d| Some(*d)).collect::<Vec<Option<i32>>>();
    // Overwrite diff[idx] with None
    new_diffs[idx] = None;
    // Calculate the new diff to the right
    if idx < diffs.len() - 1 {
        new_diffs[idx + 1] = Some(diffs[idx + 1] + diffs[idx]);
    }
    // Remove None
    new_diffs.into_iter().flatten().collect()
}

/// Check if a series of numbers is fully safe with at most one removal.
fn is_safe_with_one_removal(numbers: &[i32]) -> bool {
    // This algorithm removes each diff one at a time.
    let diffs = calculate_diffs(numbers);
    for i in 0..diffs.len() {
        let new_diffs = contract_diffs(&diffs, i);
        if diffs_are_safe(&new_diffs) {
            return true;
        }
    }
    // However, it doesn't handle the case where the first element of the original list is removed,
    // so we do a manual check of that case.
    is_safe(&numbers[1..])
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.lines() {
        let numbers = parse_line(line);
        if is_safe(&numbers) {
            safe += 1;
        }
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.lines() {
        let numbers = parse_line(line);
        if is_safe_with_one_removal(&numbers) {
            safe += 1;
        }
    }
    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
