advent_of_code::solution!(13);

use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res};

struct LinearEquations {
    // { c0*x + c1*y = c2
    // { c3*x + c4*y = c5
    c0: i64,
    c1: i64,
    c2: i64,
    c3: i64,
    c4: i64,
    c5: i64,
}

impl LinearEquations {
    fn has_linearly_dependent_coeffs(&self) -> bool {
        self.c0 * self.c4 == self.c1 * self.c3
    }

    fn solve(&self) -> Option<(i64, i64)> {
        if self.has_linearly_dependent_coeffs() {
            panic!("Linearly dependent coefficients");
            // This could mean that there are either 0 solutions (if c2/c5 != c1/c4) or infinitely
            // many solutions (if c2/c5 == c1/c4). However, the input doesn't actually contain any
            // such cases, so we can get away without handling it.
        }
        // b = (c2c3 - c5c0) / (c1c3 - c4c0)
        let b_num = self.c2 * self.c3 - self.c5 * self.c0;
        let b_denom = self.c1 * self.c3 - self.c4 * self.c0;
        if b_num % b_denom != 0 {
            // Restrict to integral solutions
            None
        } else {
            let b = b_num / b_denom;
            // a = (c2 - c1b) / c0
            let a_num = self.c2 - self.c1 * b;
            let a_denom = self.c0;
            if a_num % a_denom != 0 {
                // Again restrict to integral solutions
                None
            } else {
                let a = a_num / a_denom;
                if a > 0 && b > 0 {
                    // This doesn't happen with the input given
                    Some((a, b))
                } else {
                    None
                }
            }
        }
    }

    fn price(&self) -> u64 {
        match self.solve() {
            None => 0,
            Some((x, y)) => (x * 3 + y) as u64,
        }
    }
}

impl From<&str> for LinearEquations {
    fn from(s: &str) -> Self {
        fn parse_i64(input: &str) -> nom::IResult<&str, i64> {
            map_res(digit1, str::parse)(input)
        }
        fn parse_line<'a>(
            input: &'a str,
            first_tag: &str,
            second_tag: &str,
        ) -> nom::IResult<&'a str, (i64, i64)> {
            let (input, _) = tag(first_tag)(input)?;
            let (input, first) = parse_i64(input)?;
            let (input, _) = tag(second_tag)(input)?;
            let (input, second) = parse_i64(input)?;
            Ok((input, (first, second)))
        }
        let lines = s.lines().collect::<Vec<_>>();
        let (c0, c3) = parse_line(lines[0], "Button A: X+", ", Y+").unwrap().1;
        let (c1, c4) = parse_line(lines[1], "Button B: X+", ", Y+").unwrap().1;
        let (c2, c5) = parse_line(lines[2], "Prize: X=", ", Y=").unwrap().1;
        LinearEquations {
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = input
        .split("\n\n")
        .map(LinearEquations::from)
        .collect::<Vec<_>>();
    let total_price = equations.iter().map(LinearEquations::price).sum();
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input
        .split("\n\n")
        .map(LinearEquations::from)
        .map(|le| LinearEquations {
            c0: le.c0,
            c1: le.c1,
            c2: le.c2 + 10000000000000,
            c3: le.c3,
            c4: le.c4,
            c5: le.c5 + 10000000000000,
        })
        .collect::<Vec<_>>();
    let total_price = equations.iter().map(LinearEquations::price).sum();
    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
