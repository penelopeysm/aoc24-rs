advent_of_code::solution!(21);

use memoize::memoize;
use std::cmp::Ordering;
use std::collections::HashMap;

fn orig_keypad() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ])
}

fn robot_keypad() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ])
}

fn move_vertical(n: i32) -> Vec<char> {
    match n.cmp(&0) {
        Ordering::Equal => Vec::new(),
        Ordering::Greater => vec!['v'; n as usize],
        Ordering::Less => vec!['^'; (-n) as usize],
    }
}

fn move_horizontal(n: i32) -> Vec<char> {
    match n.cmp(&0) {
        Ordering::Equal => Vec::new(),
        Ordering::Greater => vec!['>'; n as usize],
        Ordering::Less => vec!['<'; (-n) as usize],
    }
}

fn move_h_then_v(p1: (i32, i32), p2: (i32, i32)) -> Vec<char> {
    let mut horizontal = move_horizontal(p2.1 - p1.1);
    let mut vertical = move_vertical(p2.0 - p1.0);
    horizontal.append(&mut vertical);
    horizontal
}

fn move_v_then_h(p1: (i32, i32), p2: (i32, i32)) -> Vec<char> {
    let mut vertical = move_vertical(p2.0 - p1.0);
    let mut horizontal = move_horizontal(p2.1 - p1.1);
    vertical.append(&mut horizontal);
    vertical
}

fn shortest_way(c1: char, c2: char, keypad: HashMap<char, (i32, i32)>) -> Vec<char> {
    let p1 = keypad[&c1];
    let p2 = keypad[&c2];
    let horizontal = move_horizontal(p2.1 - p1.1);
    let vertical = move_vertical(p2.0 - p1.0);
    if p1.0 == p2.0 {
        horizontal
    } else if p1.1 == p2.1 {
        vertical
    } else {
        let values = keypad.into_values().collect::<Vec<_>>();
        let can_move_h_then_v = values.contains(&(p1.0, p2.1));
        let can_move_v_then_h = values.contains(&(p2.0, p1.1));
        if p2.1 > p1.1 {
            // v>A is shorter than >vA (after it's been expanded twice)
            // ^>A is equal to     ^vA
            if can_move_v_then_h {
                move_v_then_h(p1, p2)
            } else {
                move_h_then_v(p1, p2)
            }
        } else {
            // <vA is shorter than v<A
            // <^A is shorter than v^A
            if can_move_h_then_v {
                move_h_then_v(p1, p2)
            } else {
                move_v_then_h(p1, p2)
            }
        }
    }
}

#[derive(Debug)]
struct Code {
    numeric: u64,     // e.g. 29
    chars: Vec<char>, // e.g. ['0', '2', '9', 'A']
}

impl Code {
    fn first_robot_way(&self) -> Vec<char> {
        let mut way = shortest_way('A', self.chars[0], orig_keypad());
        way.push('A');
        for i in self.chars.windows(2) {
            way.append(&mut shortest_way(i[0], i[1], orig_keypad()));
            way.push('A');
        }
        way
    }
    fn complexity(&self, n: usize) -> u64 {
        let mut first_robot_way = self.first_robot_way();
        first_robot_way.insert(0, 'A');
        let mut total_length = 0;
        for w in first_robot_way.windows(2) {
            total_length += robot_length_at_depth(w[0], w[1], n);
        }
        total_length * self.numeric
    }
}

#[memoize]
fn robot_length_at_depth(c1: char, c2: char, n: usize) -> u64 {
    // Find the shortest path to move from c1 to c2
    let mut shortest_way = shortest_way(c1, c2, robot_keypad());
    // The shortest path doesn't include an 'A', which the next robot needs to press
    shortest_way.push('A');
    if n == 1 {
        // If at depth 1, then the number of keypresses needed to get from
        // c1 to c2 is just the length of the shortest path plus its A
        shortest_way.len() as u64
    } else {
        // Otherwise, suppose it gives us a sequence of characters like >>A. To press these
        // buttons, the next robot will need to go (1) from A to >, (2) from > to >, and (3) from >
        // to A. (Note that regardless of where in the _previous_ sequence this '>>A' came from,
        // the robot always starts at A, either because that is its initial position, or because
        // the previous sequence of inputs would have returned it to A.) Thus we can recursively
        // call this function, and the memoisation ensures that it is efficient.
        let mut total_length = 0;
        shortest_way.insert(0, 'A');
        for w in shortest_way.windows(2) {
            total_length += robot_length_at_depth(w[0], w[1], n - 1);
        }
        total_length
    }
}

// For debugging purposes
fn _unstep_robot(way: String, keypad: HashMap<char, (i32, i32)>) -> String {
    let mut undone = Vec::new();
    let mut starting_point = keypad[&'A'];
    for c in way.chars() {
        match c {
            '^' => starting_point.0 -= 1,
            'v' => starting_point.0 += 1,
            '<' => starting_point.1 -= 1,
            '>' => starting_point.1 += 1,
            _ => {
                for (c, p) in &keypad {
                    if *p == starting_point {
                        undone.push(*c);
                        break;
                    }
                }
            }
        }
    }
    undone.iter().collect()
}
// For debugging purposes
fn _next_robot_way(chars: Vec<char>) -> Vec<char> {
    let mut way = shortest_way('A', chars[0], orig_keypad());
    way.push('A');
    for i in chars.windows(2) {
        way.append(&mut shortest_way(i[0], i[1], orig_keypad()));
        way.push('A');
    }
    way
}
// For debugging purposes
fn _step_robot(prev_step: String) -> String {
    let prev_step = prev_step.chars().collect::<Vec<_>>();
    let way = _next_robot_way(prev_step);
    way.iter().collect()
}

fn parse_input(input: &str) -> Vec<Code> {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect();
            let numeric = line[0..line.len() - 1].parse().unwrap();
            Code { numeric, chars }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let codes = parse_input(input);
    Some(codes.into_iter().map(|code| code.complexity(2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let codes = parse_input(input);
    Some(codes.into_iter().map(|code| code.complexity(25)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
