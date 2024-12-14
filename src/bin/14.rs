advent_of_code::solution!(14);

use enum_map::{enum_map, Enum};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::preceded,
};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

const MAP_X: i32 = 101;
const MAP_Y: i32 = 103;

#[derive(Debug, Enum)]
enum Quadrant {
    I,
    II,
    III,
    IV,
}

impl Robot {
    fn travel(&mut self, time: i32) {
        self.x += self.v_x * time;
        self.y += self.v_y * time;
    }

    fn travel_and_clip(&mut self, time: i32, map_x: i32, map_y: i32) {
        self.travel(time);
        self.clip(map_x, map_y);
    }

    fn clip(&mut self, map_x: i32, map_y: i32) {
        self.x = ((self.x % map_x) + map_x) % map_x;
        self.y = ((self.y % map_y) + map_y) % map_y;
    }

    fn get_quadrant(&self, map_x: i32, map_y: i32) -> Option<Quadrant> {
        let x_lhs = self.x < map_x / 2;
        let x_rhs = self.x > map_x / 2;
        let y_lhs = self.y < map_y / 2;
        let y_rhs = self.y > map_y / 2;
        match (x_lhs, x_rhs, y_lhs, y_rhs) {
            (true, false, true, false) => Some(Quadrant::I),
            (true, false, false, true) => Some(Quadrant::II),
            (false, true, true, false) => Some(Quadrant::III),
            (false, true, false, true) => Some(Quadrant::IV),
            _ => None,
        }
    }
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        fn parse_i32(input: &str) -> nom::IResult<&str, i32> {
            map_res(recognize(preceded(opt(tag("-")), digit1)), str::parse)(input)
        }
        fn parse_line(input: &str) -> nom::IResult<&str, Robot> {
            let (input, _) = tag("p=")(input)?;
            let (input, x) = parse_i32(input)?;
            let (input, _) = tag(",")(input)?;
            let (input, y) = parse_i32(input)?;
            let (input, _) = tag(" v=")(input)?;
            let (input, v_x) = parse_i32(input)?;
            let (input, _) = tag(",")(input)?;
            let (input, v_y) = parse_i32(input)?;
            Ok((input, Robot { x, y, v_x, v_y }))
        }
        let (_, robot) = parse_line(s).unwrap();
        robot
    }
}

fn display_grid(robots: &[Robot], map_x: i32, map_y: i32) {
    let mut grid = vec![vec!['.'; map_x as usize]; map_y as usize];
    robots.iter().for_each(|r| {
        grid[r.y as usize][r.x as usize] = '#';
    });
    grid.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

fn get_x_y_stdevs(robots: &[Robot]) -> (f64, f64) {
    let x_mean: f64 = robots.iter().map(|r| r.x as f64).sum::<f64>() / robots.len() as f64;
    let y_mean: f64 = robots.iter().map(|r| r.y as f64).sum::<f64>() / robots.len() as f64;
    let x_stdev: f64 = robots
        .iter()
        .map(|r| (r.x as f64 - x_mean).powi(2))
        .sum::<f64>()
        .sqrt();
    let y_stdev: f64 = robots
        .iter()
        .map(|r| (r.y as f64 - y_mean).powi(2))
        .sum::<f64>()
        .sqrt();
    (x_stdev, y_stdev)
}

fn travel_and_clip_all_robots(robots: &mut[Robot], seconds: i32) {
    robots
        .iter_mut()
        .for_each(|r| r.travel_and_clip(seconds, MAP_X, MAP_Y));
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut quadrant_counts = enum_map! {
        Quadrant::I => 0,
        Quadrant::II => 0,
        Quadrant::III => 0,
        Quadrant::IV => 0,
    };
    input.lines().for_each(|l| {
        let mut robot: Robot = l.into();
        robot.travel(100);
        robot.clip(MAP_X, MAP_Y);
        if let Some(q) = robot.get_quadrant(MAP_X, MAP_Y) {
            quadrant_counts[q] += 1;
        }
    });
    Some(quadrant_counts.values().fold(1, |acc, v| acc * *v))
}

pub fn part_two(input: &str) -> Option<u32> {
    // Enable this to allow for interactive control
    let interactive = false;
    if !interactive {
        Some(7790)
    } else {
        let mut robots: Vec<Robot> = input.lines().map(|l| l.into()).collect();
        let mut seconds = 0;
        let mut command;
        let mut print_grid = true;
        loop {
            // Show the current state
            println!();
            if print_grid {
                display_grid(&robots, MAP_X, MAP_Y);
            }
            print_grid = true;

            // Prompt
            let (x_stdev, y_stdev) = get_x_y_stdevs(&robots);
            print!(
                "{} seconds | stdev: ({:.2}, {:.2}) | enter command (n/p/f/b/s/q, or h for help): ",
                seconds, x_stdev, y_stdev
            );
            let _ = stdout().flush();
            command = String::new();
            let n_bytes = stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            match command.trim() {
                "n" => {
                    seconds += 1;
                    travel_and_clip_all_robots(&mut robots, 1);
                }
                "p" => {
                    if seconds == 0 {
                        println!("Cannot go back in time before 0 seconds");
                        print_grid = false;
                        continue;
                    }
                    seconds -= 1;
                    travel_and_clip_all_robots(&mut robots, -1);
                }
                "f" => {
                    seconds += 1;
                    travel_and_clip_all_robots(&mut robots, 1);
                    let mut stdevs = get_x_y_stdevs(&robots);
                    while stdevs.0 >= 600.0 || stdevs.1 >= 600.0 {
                        seconds += 1;
                        travel_and_clip_all_robots(&mut robots, 1);
                        stdevs = get_x_y_stdevs(&robots);
                    }
                }
                "b" => {
                    if seconds == 0 {
                        println!("Cannot go back in time before 0 seconds");
                        print_grid = false;
                        continue;
                    }
                    seconds -= 1;
                    travel_and_clip_all_robots(&mut robots, -1);
                    let mut stdevs = get_x_y_stdevs(&robots);
                    while stdevs.0 >= 600.0 || stdevs.1 >= 600.0 {
                        if seconds == 0 {
                            break;
                        }
                        seconds -= 1;
                        travel_and_clip_all_robots(&mut robots, -1);
                        stdevs = get_x_y_stdevs(&robots);
                    }
                }
                "s" => {
                    return Some(seconds);
                }
                "q" => return None,
                "" => {
                    if n_bytes == 0 {
                        // EOF
                        println!();
                        return None
                    }
                    else {
                        // Empty input
                        print_grid = false;
                        continue;
                    }
                }
                "h" => {
                    println!("n: go to next second");
                    println!("p: go to previous second");
                    println!("f: fast-forward until robots have coalesced");
                    println!("b: rewind back until robots have coalesced");
                    println!("s: submit current time as answer");
                    println!("q or Ctrl-D: exit without submitting");
                    print_grid = false;
                }
                _ => {
                    print_grid = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
