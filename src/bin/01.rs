use std::collections::HashMap;

advent_of_code::solution!(1);


fn parse_input_and_sort(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().map(|n| n.parse()).collect::<Vec<Result<u32,_>>>();
        match parts[..] {
            [Ok(x), Ok(y)] => {
                xs.push(x);
                ys.push(y);
            },
            _ => panic!("Invalid input, expected two positive integers per line"),
        }
    }
    xs.sort();
    ys.sort();
    (xs, ys)
}

fn parse_input_into_count_maps(input: &str) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut x_counts = HashMap::new();
    let mut y_counts = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().map(|n| n.parse()).collect::<Vec<Result<u32,_>>>();
        match parts[..] {
            [Ok(x), Ok(y)] => {
                x_counts.entry(x).and_modify(|e| *e += 1).or_insert(1);
                y_counts.entry(y).and_modify(|e| *e += 1).or_insert(1);
            },
            _ => panic!("Invalid input, expected two positive integers per line"),
        }
    }
    (x_counts, y_counts)
}


pub fn part_one(input: &str) -> Option<u32> {
    let (sorted_xs, sorted_ys) = parse_input_and_sort(input);
    let mut acc = 0;
    for (x, y) in std::iter::zip(sorted_xs, sorted_ys) {
        acc += x.abs_diff(y);
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (x_counts, y_counts) = parse_input_into_count_maps(input);
    // For each item in the left list, see how many times it appears in the
    // left list, and multiply by the number of times it appears in the right list
    let mut acc = 0;
    for (key, val) in x_counts.iter() {
        let n_left = val;
        let n_right = y_counts.get(key).unwrap_or(&0);
        acc += key * n_left * n_right;
    }
    Some(acc)
// }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
