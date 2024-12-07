advent_of_code::solution!(7);

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace0,
    combinator::map_res, multi::many1, sequence::terminated, IResult,
};
use std::collections::HashMap;

struct Countdown {
    target: u64,
    numbers: Vec<u64>,
    cache: HashMap<Vec<Op>, u64>, // Mapping of operators -> results
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Countdown {
    /// For a series of operators ops[0], ops[1], ..., ops[i], calculate
    /// the value of the expression (self.numbers[0] ops[0] self.numbers[1]
    /// ops[1] ... ops[i] self.numbers[i+1]), with left associativity.
    ///
    /// Note that the list of operators may be shorter than the list of numbers
    /// (minus 1), in which case the numbers are not fully used.
    ///
    /// Returns None if the result is known to be larger than self.target.
    /// Otherwise returns Some(result).
    fn calculate(&mut self, ops: &[Op]) -> Option<u64> {
        // Base case: if the ops are empty, just return the first number
        if ops.is_empty() {
            self.cache.insert(ops.to_vec(), self.numbers[0]);
            return Some(self.numbers[0]);
        }

        // If it's already been cached, return it
        if let Some(v) = self.cache.get(ops) {
            return Some(*v);
        }

        // Otherwise, recurse

        // Notice that the operations involved only lead to the accumulator
        // getting larger. So, if our accumulator is already larger than the
        // target, we can just shortcircuit and return None. This cuts about
        // 20-30% off the time.
        let acc = self.calculate(&ops[..ops.len() - 1]);
        match acc {
            // The previous calculation was already larger than the target
            None => None,

            Some(a) => {
                let next = self.numbers[ops.len()];
                let result = match ops[ops.len() - 1] {
                    Op::Add => a + next,
                    Op::Mul => a * next,
                    Op::Concat => {
                        let n_digits_next = next.to_string().len();
                        a * 10u64.pow(n_digits_next as u32) + next
                    }
                };
                if result > self.target {
                    None
                }
                else {
                    self.cache.insert(ops.to_vec(), result);
                    Some(result)
                }
            }
        }
    }

    /// Given a list of allowed operators allowed_ops, find a combination
    /// of operators ops for which self.calculate(ops) == self.target and every
    /// element of ops is in allowed_ops.
    ///
    /// Note: The allowed operations must always lead to a result that is larger
    /// than both of its operands. Thus, if an operator like Minus is passed,
    /// this function may not work correctly.
    /// 
    /// If there is no solution, returns None. Otherwise, returns Some((ops, target)),
    /// where ops is the list of operators that lead to the target.
    fn solve(mut self, allowed_ops: Vec<Op>) -> Option<(Vec<Op>, u64)> {
        let n = self.numbers.len() - 1;
        let allowed_ops_combinations = (0..n).map(|_| allowed_ops.clone().into_iter());
        allowed_ops_combinations
            .multi_cartesian_product()
            .find(|ops| self.calculate(&ops[..]) == Some(self.target))
            .map(|maybe_sol| (maybe_sol, self.target))
    }
}

impl From<&str> for Countdown {
    fn from(input_line: &str) -> Self {
        fn parse_number(input: &str) -> IResult<&str, u64> {
            map_res(terminated(digit1, multispace0), |s: &str| s.parse::<u64>())(input)
        }
        fn parse(input: &str) -> IResult<&str, Countdown> {
            let (input, target) = parse_number(input)?;
            let (input, _) = tag(": ")(input)?;
            let (input, numbers) = many1(parse_number)(input)?;
            assert!(input.is_empty());
            Ok((
                input,
                Countdown {
                    target,
                    numbers,
                    cache: HashMap::new(),
                },
            ))
        }
        match parse(input_line) {
            Ok((_, c)) => c,
            Err(e) => panic!("Failed to parse: {:?}", e),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| Countdown::from(l).solve(vec![Op::Add, Op::Mul]))
            .fold(0, |acc, res| match res {
                Some((_, target)) => acc + target,
                None => acc,
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| Countdown::from(l).solve(vec![Op::Add, Op::Mul, Op::Concat]))
            .fold(0, |acc, res| match res {
                Some((_, target)) => acc + target,
                None => acc,
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
