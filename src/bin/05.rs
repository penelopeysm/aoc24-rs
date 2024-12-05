advent_of_code::solution!(5);

use std::collections::HashSet;

type Page = u32;

#[derive(Clone, Debug)]
struct RulePage {
    earlier: Page,
    later: Page,
}

impl From<&str> for RulePage {
    fn from(s: &str) -> Self {
        let parts: Vec<Page> = s.split('|').map(|i| i.parse().unwrap()).collect();
        match parts[..] {
            [earlier, later] => Self { earlier, later },
            _ => panic!("Invalid rule page"),
        }
    }
}

#[derive(Debug)]
struct RuleIndex {
    earlier: usize,
    later: usize,
}

/// Returns None if either page is not in the list of pages
fn to_ruleindex(rp: &RulePage, pages: &[Page]) -> Option<RuleIndex> {
    let page1 = pages.iter().position(|&p| p == rp.earlier);
    let page2 = pages.iter().position(|&p| p == rp.later);
    match (page1, page2) {
        (Some(p1), Some(p2)) => Some(RuleIndex {
            earlier: p1,
            later: p2,
        }),
        _ => None,
    }
}

fn is_rule_obeyed(ri: RuleIndex) -> bool {
    ri.earlier < ri.later
}

fn parse_input(input: &str) -> (Vec<RulePage>, Vec<Vec<Page>>) {
    if let [rules_str, pages_str] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        let rules = rules_str.lines().map(|s| s.into()).collect();
        let pages = pages_str
            .lines()
            .map(|s| s.split(',').map(|i| i.parse().unwrap()).collect())
            .collect();
        return (rules, pages);
    }
    panic!("Invalid input");
}

fn get_rule_indices(pages: &[Page], rules: &[RulePage]) -> Vec<RuleIndex> {
    rules
        .iter()
        // Parse indices
        .map(|rp| to_ruleindex(rp, pages))
        .collect::<Vec<Option<RuleIndex>>>()
        // Remove Nones (i.e. inapplicable rules)
        .into_iter()
        .flatten()
        .collect::<Vec<RuleIndex>>()
}

fn pages_are_legal(pages: &[Page], rules: &[RulePage]) -> bool {
    let rule_indices = get_rule_indices(pages, rules);
    rule_indices
        .into_iter()
        .all(is_rule_obeyed)
}

fn get_middle(pages: &[Page]) -> Page {
    pages[(pages.len() - 1) / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rulepages, pages) = parse_input(input);
    Some(
        pages
            .iter()
            .filter(|p| pages_are_legal(p, &rulepages))
            .map(|p| get_middle(p))
            .sum(),
    )
}

fn get_sorted_indices(rule_indices: Vec<RuleIndex>) -> Vec<usize> {
    // If there is only one rule, return the indices in order
    if rule_indices.len() == 1 {
        return vec![rule_indices[0].earlier, rule_indices[0].later];
    }

    // Otherwise, we find the first index and then recurse

    // Get all indices that appear in the rules
    let all_indices: HashSet<usize> = rule_indices.iter().flat_map(|ri| vec![ri.earlier, ri.later]).collect();
    // Find an index for which no index should come before it
    let later_indices: HashSet<usize> = rule_indices.iter().map(|ri| ri.later).collect();
    let indices_without_later: Vec<&usize> = all_indices.difference(&later_indices).collect();
    assert!(!indices_without_later.is_empty()); // If we hit this, the problem is unsolvable
    let first_index = indices_without_later[0];
    // Filter out all rules involving the first index
    let remaining_rules: Vec<RuleIndex> = rule_indices
        .into_iter()
        .filter(|ri| ri.earlier != *first_index)
        .collect();
    let mut remaining_indices = get_sorted_indices(remaining_rules);
    // Insert the first index at the beginning
    remaining_indices.insert(0, *first_index);
    remaining_indices
}

fn get_middle_index_of_sorted(pages: &[Page], rules: &[RulePage]) -> usize {
    let rule_indices = get_rule_indices(pages, rules);
    let sorted_indices = get_sorted_indices(rule_indices);
    sorted_indices[(sorted_indices.len() - 1) / 2]
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rulepages, pages) = parse_input(input);
    Some(
        pages
            .iter()
            .filter(|p| !pages_are_legal(p, &rulepages))
            .map(|p| p[get_middle_index_of_sorted(p, &rulepages)])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
