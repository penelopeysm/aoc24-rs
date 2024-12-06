advent_of_code::solution!(6);

use itertools::iproduct;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Square {
    Visited,
    NotVisited,
    Obstacle,
}

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    index: (usize, usize),
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum TerminationCondition {
    OutOfBounds,
    HitLoop,
}

fn get_next_index(
    nrows: usize,
    ncols: usize,
    cur_idx: (usize, usize),
    direction: &Direction,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if cur_idx.0 == 0 {
                None
            } else {
                Some((cur_idx.0 - 1, cur_idx.1))
            }
        }
        Direction::Down => {
            if cur_idx.0 == nrows - 1 {
                None
            } else {
                Some((cur_idx.0 + 1, cur_idx.1))
            }
        }
        Direction::Left => {
            if cur_idx.1 == 0 {
                None
            } else {
                Some((cur_idx.0, cur_idx.1 - 1))
            }
        }
        Direction::Right => {
            if cur_idx.1 == ncols - 1 {
                None
            } else {
                Some((cur_idx.0, cur_idx.1 + 1))
            }
        }
    }
}

fn rotate_right(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

#[derive(Clone)]
struct LabMap {
    board: Vec<Vec<(Square, HashSet<Direction>)>>,
    nrows: usize,
    ncols: usize,
    guard: Guard,
    terminated: Option<TerminationCondition>,
}

// Take a step in the direction the guard is facing.
//
// If the step would cause the guard to move out of bounds, return the
// same map and Some(OutOfBounds).
//
// If the step would cause the guard to enter a loop, return the same
// map and Some(HitLoop).
//
// Otherwise, return the updated map and None
fn _step_1_and_check(mut map: LabMap) -> LabMap {
    // Check for out of bounds
    let next_index = get_next_index(map.nrows, map.ncols, map.guard.index, &map.guard.direction);
    match next_index {
        None => {
            map.terminated = Some(TerminationCondition::OutOfBounds);
            map
        }
        // Move the guard
        Some((x, y)) => {
            match map.board[x][y] {
                (Square::Obstacle, _) => {
                    map.guard.direction = rotate_right(&map.guard.direction);
                    map.board[map.guard.index.0][map.guard.index.1]
                        .1
                        .insert(map.guard.direction.clone());
                }
                _ => {
                    map.guard.index = (x, y);
                    // Check for loop
                    if map.board[x][y].1.contains(&map.guard.direction) {
                        map.terminated = Some(TerminationCondition::HitLoop);
                        return map;
                    }
                    // If no loop, proceed
                    map.board[x][y].1.insert(map.guard.direction.clone());
                    map.board[x][y].0 = Square::Visited;
                }
            }
            map
        }
    }
}

// Same behaviour as _step_1_and_check, but attempts to move multiple squares at a time. It leads to
// around a 10% speedup.
fn step_n_and_check(mut map: LabMap) -> LabMap {
    // Get the indices of all unblocked squares in front of the guard,
    // together with a flag to indicate whether it terminates by going
    // out of bounds
    let mut next_indices = Vec::new();
    let mut out_of_bounds = false;
    let mut next_index =
        get_next_index(map.nrows, map.ncols, map.guard.index, &map.guard.direction);
    loop {
        match next_index {
            // If moved out of bounds, set flag to true.
            None => {
                out_of_bounds = true;
                break;
            }
            Some((x, y)) => {
                // Break if hit an obstacle
                if map.board[x][y].0 == Square::Obstacle {
                    break;
                }
                // Else add the index
                else {
                    next_indices.push((x, y));
                    next_index = get_next_index(map.nrows, map.ncols, (x, y), &map.guard.direction);
                }
            }
        }
    }
    // Set the squares to visited + add the directions
    for (x, y) in &next_indices[..] {
        // Check if any loops are hit
        if map.board[*x][*y].1.contains(&map.guard.direction) {
            map.terminated = Some(TerminationCondition::HitLoop);
            return map;
        }
        // If not, add them
        map.board[*x][*y].1.insert(map.guard.direction.clone());
        map.board[*x][*y].0 = Square::Visited;
    }
    // If out of bounds, terminate
    if out_of_bounds {
        map.terminated = Some(TerminationCondition::OutOfBounds);
        map
    }
    // If not out of bounds, means we hit an obstacle. Rotate right and continue
    else {
        map.guard.direction = rotate_right(&map.guard.direction);
        match next_indices.last() {
            None => {}
            Some((x, y)) => {
                map.guard.index = (*x, *y);
            }
        }
        map.board[map.guard.index.0][map.guard.index.1]
            .1
            .insert(map.guard.direction.clone());
        map
    }
}

fn run(mut map: LabMap) -> LabMap {
    // Take one step
    map = step_n_and_check(map);
    match map.terminated {
        Some(_) => map,
        None => run(map),
    }
}

fn count_visited(map: LabMap) -> usize {
    map.board
        .iter()
        .flatten()
        .filter(|x| x.0 == Square::Visited)
        .count()
}

fn parse_input(input: &str) -> LabMap {
    fn parse_char(input: char) -> (Square, Option<Direction>) {
        match input {
            '#' => (Square::Obstacle, None),
            '.' => (Square::NotVisited, None),
            '^' => (Square::Visited, Some(Direction::Up)),
            'v' => (Square::Visited, Some(Direction::Down)),
            '<' => (Square::Visited, Some(Direction::Left)),
            '>' => (Square::Visited, Some(Direction::Right)),
            _ => panic!("Invalid character in input"),
        }
    }
    let mut board = Vec::new();
    let mut guard = None;
    let mut nrows = 0;
    let mut ncols = 0;
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let (square, dir) = parse_char(c);
            row.push((square, HashSet::new()));
            // Check for a guard
            if let Some(d) = dir {
                assert!(guard.is_none(), "Multiple guards in input");
                guard = Some(Guard {
                    direction: d.clone(),
                    index: (i, j),
                });
                row[j].1.insert(d);
            };
            ncols = j + 1;
        }
        nrows += 1;
        board.push(row);
    }
    match guard {
        None => panic!("No guard in input"),
        Some(g) => LabMap {
            board,
            nrows,
            ncols,
            guard: g,
            terminated: None,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let final_map = run(map);
    Some(count_visited(final_map) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut n_loops = 0;
    // // This is inefficient as it checks every single square
    // for (i, j) in iproduct!(0..map.nrows, 0..map.ncols) {
    //     if map.map[i][j].0 == Square::NotVisited {
    //         let mut map = map.clone();
    //         map.map[i][j].0 = Square::Obstacle;
    //         let termination_condition = run(map).terminated;
    //         if termination_condition == TerminationCondition::HitLoop {
    //             println!("Loop detected at ({}, {})", i, j);
    //             n_loops += 1;
    //         }
    //     }
    // }

    // This is somewhat more efficient. It takes about 19.1 s with the dev
    // profile, but around 1.4 s with release profile. I suspect that the
    // algorithm still isn't optimal, but eh.

    // Run the map once to get the trajectory. We use this to determine the
    // set of possible locations where adding an obstacle could affect the
    // trajectory.
    let final_map = run(map.clone());
    let mut possible_obstacles = HashSet::new();
    for (i, j) in iproduct!(0..map.nrows, 0..map.ncols) {
        if let (Square::Visited, dirs) = &final_map.board[i][j] {
            for dir in dirs {
                let possible_obstacle_index = get_next_index(map.nrows, map.ncols, (i, j), dir);
                if let Some((x, y)) = possible_obstacle_index {
                    if final_map.board[x][y].0 != Square::Obstacle {
                        possible_obstacles.insert((x, y));
                    }
                }
            }
        }
    }
    // Then we iterate through that set, instead of the entire map
    for (i, j) in possible_obstacles {
        let mut map = map.clone();
        map.board[i][j].0 = Square::Obstacle;
        if run(map).terminated == Some(TerminationCondition::HitLoop) {
            // println!("Loop detected at ({}, {})", i, j);
            n_loops += 1;
        }
    }

    Some(n_loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
