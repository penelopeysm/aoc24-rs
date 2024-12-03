use regex::Regex;

advent_of_code::solution!(3);

/// Regex solution
///
/// Benchmark:
/// Day 03
/// ------
/// Part 1: 189600467 (309.7µs @ 476 samples)
/// Part 2: 107069718 (334.3µs @ 2065 samples)

struct Mul {
    a: u32,
    b: u32,
}

impl From<(&str, &str)> for Mul {
    fn from((a_str, b_str): (&str, &str)) -> Self {
        let a = a_str.parse().unwrap();
        let b = b_str.parse().unwrap();
        Mul { a, b }
    }
}

fn sum_muls(muls: &[Mul]) -> u32 {
    muls.iter().map(|mul| mul.a * mul.b).sum()
}

fn extract_muls(input: &str) -> Vec<Mul> {
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
    let mut muls = vec![];
    for (_, [a_str, b_str]) in re.captures_iter(input).map(|c| c.extract()) {
        muls.push((a_str, b_str).into());
    }
    muls
}

// NOTE: This is VERY unsatisfactory compared to a parser, especially with the extra capture groups
// at the end
fn extract_muls_with_instructions(input: &str) -> Vec<Mul> {
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)|do\(\)(())|don't\(\)(())"#).unwrap();
    let mut muls = vec![];
    let mut enabled = true;
    for (substr, [a_str, b_str]) in re.captures_iter(input).map(|c| c.extract()) {
        if substr == "do()" {
            enabled = true;
        } else if substr == "don't()" {
            enabled = false;
        } else if enabled {
            muls.push((a_str, b_str).into());
        }
    }
    muls
}

/// Parser combinator solution (3 times faster!)
///
/// Benchmark:
/// Part 1: 189600467 (111.5µs @ 2713 samples)
/// Part 2: 107069718 (100.5µs @ 9374 samples)
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::anychar,
    combinator::{map_res, value},
    multi::many1,
    IResult, Parser,
};

#[derive(Clone)]
enum Instruction {
    Mull(u32, u32), // Using a different identifier from above for clarity
    Do,
    Dont,
    AnyOtherChar, // Will be ignored
}

fn parse_1_to_3_digit_num(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(1, 3, |c: char| c.is_numeric()), |s: &str| {
        s.parse::<u32>()
    })
    .parse(input)
}

fn parse_mull(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = parse_1_to_3_digit_num(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = parse_1_to_3_digit_num(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Instruction::Mull(a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_char(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::AnyOtherChar, anychar).parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((parse_mull, parse_do, parse_dont, parse_char))).parse(input)
}

fn collapse_part1(instructions: &[Instruction]) -> u32 {
    let mut sum = 0;
    for inst in instructions {
        if let Instruction::Mull(a, b) = inst {
            sum += a * b
        };
    }
    sum
}

fn collapse_part2(instructions: &[Instruction]) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for inst in instructions {
        match inst {
            Instruction::Mull(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            _ => {}
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let (remaining_input, instructions) = parse(input).expect("Failed to parse input");
    assert!(remaining_input.is_empty());
    Some(collapse_part1(&instructions))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (remaining_input, instructions) = parse(input).expect("Failed to parse input");
    assert!(remaining_input.is_empty());
    Some(collapse_part2(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
