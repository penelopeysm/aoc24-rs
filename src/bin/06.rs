advent_of_code::solution!(6);

use itertools::iproduct;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Square {
    NotObstacle(HashSet<Direction>),
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
    board: Vec<Vec<Square>>,
    nrows: usize,
    ncols: usize,
    guard: Guard,
    terminated: Option<TerminationCondition>,
}

impl LabMap {
    /// Take a step in the direction the guard is facing.
    ///
    /// If the step would cause the guard to move out of bounds, set
    /// self.terminated to Some(OutOfBounds). If the step would cause the guard to enter a loop,
    /// set it to Some(HitLoop).
    fn step1(&mut self) {
        // Add current square and direction to the visited. Also check for loops
        if let Square::NotObstacle(visited) =
            &mut self.board[self.guard.index.0][self.guard.index.1]
        {
            if visited.contains(&self.guard.direction) {
                self.terminated = Some(TerminationCondition::HitLoop);
                return;
            } else {
                visited.insert(self.guard.direction.clone());
            }
        }

        let next_index = get_next_index(
            self.nrows,
            self.ncols,
            self.guard.index,
            &self.guard.direction,
        );
        match next_index {
            // Out of bounds
            None => {
                self.terminated = Some(TerminationCondition::OutOfBounds);
            }
            // In bounds, so move the guard appropriately
            Some((x, y)) => match self.board[x][y] {
                Square::Obstacle => {
                    self.guard.direction = rotate_right(&self.guard.direction);
                }
                _ => {
                    self.guard.index = (x, y);
                }
            },
        }
    }

    fn run(&mut self) {
        while self.terminated.is_none() {
            self.step1();
        }
    }

    fn count_visited(self) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|x| match x {
                Square::Obstacle => false,
                Square::NotObstacle(visited) => !visited.is_empty(),
            })
            .count()
    }
}

impl From<&str> for LabMap {
    fn from(input: &str) -> Self {
        fn parse_char(input: char) -> (Square, Option<Direction>) {
            match input {
                '#' => (Square::Obstacle, None),
                '.' => (Square::NotObstacle(HashSet::new()), None),
                '^' => (Square::NotObstacle(HashSet::new()), Some(Direction::Up)),
                'v' => (Square::NotObstacle(HashSet::new()), Some(Direction::Down)),
                '<' => (Square::NotObstacle(HashSet::new()), Some(Direction::Left)),
                '>' => (Square::NotObstacle(HashSet::new()), Some(Direction::Right)),
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
                let (square, maybe_guard_dir) = parse_char(c);
                // Check for a guard
                match maybe_guard_dir {
                    None => {}
                    Some(d) => {
                        assert!(guard.is_none(), "Multiple guards in input");
                        guard = Some(Guard {
                            direction: d.clone(),
                            index: (i, j),
                        });
                    }
                };
                row.push(square);
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: LabMap = input.into();
    map.run();
    Some(map.count_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: LabMap = input.into();
    let mut n_loops = 0;

    // Run the map once to get the trajectory. We use this to determine the
    // set of possible locations where adding an obstacle could affect the
    // trajectory.
    let mut final_map = map.clone();
    final_map.run();
    let mut possible_obstacles = HashSet::new();
    for (i, j) in iproduct!(0..final_map.nrows, 0..final_map.ncols) {
        if let Square::NotObstacle(dirs) = &final_map.board[i][j] {
            for dir in dirs {
                let possible_obstacle_index =
                    get_next_index(final_map.nrows, final_map.ncols, (i, j), dir);
                if let Some((x, y)) = possible_obstacle_index {
                    if final_map.board[x][y] != Square::Obstacle {
                        possible_obstacles.insert((x, y));
                    }
                }
            }
        }
    }
    // println!("{:?}", possible_obstacles);

    // Then we iterate through that set, instead of the entire map
    for (i, j) in possible_obstacles {
        // println!("Testing ({}, {})", i, j);
        let mut new_map = map.clone();
        new_map.board[i][j] = Square::Obstacle;
        new_map.run();
        if new_map.terminated == Some(TerminationCondition::HitLoop) {
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
