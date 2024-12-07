advent_of_code::solution!(6);

use itertools::iproduct;

#[derive(Clone, PartialEq)]
enum Square {
    NotObstacle(u32), // Number of times this square has been visited
    Obstacle,
}

impl Square {
    fn has_been_visited(&self) -> bool {
        match self {
            Square::Obstacle => false,
            Square::NotObstacle(visited) => *visited > 0,
        }
    }
}

#[derive(PartialEq, Hash, Eq, PartialOrd, Ord, Clone)]
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
    nrows: usize, // Just to avoid recomputation
    ncols: usize,
    guard: Guard,
    terminated: Option<TerminationCondition>,
}

impl LabMap {
    // Increment the counter for the square the guard is currently on.
    fn track_direction(&mut self) {
        if let Square::NotObstacle(visited) =
            &mut self.board[self.guard.index.0][self.guard.index.1]
        {
            // By the pigeonhole principle, if we've visited a square more than 4 times, at least 2
            // of those times must have been in the same direction. This means we've hit a loop.
            //
            // In fact, if the square being tracked is next to an obstacle, it suffices to check
            // *visited > 3 because it's impossible to visit a square next to an obstacle in all
            // four directions. So, for part 2 we could optimise this slightly, but it's not
            // generalisable and leads to negligible speedup, so I'll leave it.
            if *visited > 4 {
                self.terminated = Some(TerminationCondition::HitLoop);
            } else {
               *visited += 1;
            }
        } else {
            panic!("Guard is on an obstacle");
        }
    }

    /// Take a step in the direction the guard is facing.
    ///
    /// If the step would cause the guard to move out of bounds, set self.terminated to
    /// Some(OutOfBounds). If the step would cause the guard to enter a loop, set it to
    /// Some(HitLoop).
    ///
    /// The `full` parameter determines whether tracking is enabled for every square the guard
    /// visits, or just the squares immediately before obstacles. For part 1 of this problem, we
    /// need to track every square; however, for part 2 (to determine loops), we only need to track
    /// the squares immediately before obstacles to see whether they are revisited multiple times.
    /// This results in a ~ 2.5x speedup.
    ///
    /// I didn't come up with this idea myself unfortunately, I saw it on Reddit.
    fn step1(&mut self, full: bool) {
        // Add current square and direction to the visited. Also check for loops
        if full {
            self.track_direction();
            if self.terminated.is_some() {
                return;
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
                    if !full {
                        self.track_direction();
                    }
                    self.guard.direction = rotate_right(&self.guard.direction);
                }
                _ => {
                    self.guard.index = (x, y);
                }
            },
        }
    }

    fn run(&mut self, full: bool) {
        while self.terminated.is_none() {
            self.step1(full);
        }
    }

    fn count_visited(self) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|x| x.has_been_visited())
            .count()
    }
}

impl From<&str> for LabMap {
    fn from(input: &str) -> Self {
        fn parse_char(input: char) -> (Square, Option<Direction>) {
            fn not_obstacle() -> Square {
                Square::NotObstacle(0)
            }
            match input {
                '#' => (Square::Obstacle, None),
                '.' => (not_obstacle(), None),
                '^' => (not_obstacle(), Some(Direction::Up)),
                'v' => (not_obstacle(), Some(Direction::Down)),
                '<' => (not_obstacle(), Some(Direction::Left)),
                '>' => (not_obstacle(), Some(Direction::Right)),
                _ => panic!("Invalid character in input"),
            }
        }
        let mut board = Vec::new();
        let mut guard = None;
        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, char) in line.chars().enumerate() {
                let (square, maybe_guard_dir) = parse_char(char);
                // Check for a guard
                match maybe_guard_dir {
                    None => {}
                    Some(d) => {
                        assert!(guard.is_none(), "Multiple guards in input");
                        guard = Some(Guard {
                            direction: d,
                            index: (i, j),
                        });
                    }
                };
                // We don't need to set the visited state for the initial square, because step1()
                // will do it for us on the first iteration
                row.push(square);
            }
            board.push(row);
        }
        let nrows = &board.len();
        let ncols = &board[0].len();
        match guard {
            None => panic!("No guard in input"),
            Some(g) => LabMap {
                board,
                nrows: *nrows,
                ncols: *ncols,
                guard: g,
                terminated: None,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: LabMap = input.into();
    map.run(true);
    Some(map.count_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: LabMap = input.into();

    // Run the map once to get the trajectory. We use this to determine the
    // set of possible locations where adding an obstacle could affect the
    // trajectory.
    let mut final_map = map.clone();
    final_map.run(true);
    let possible_obstacles = iproduct!(0..final_map.nrows, 0..final_map.ncols)
        .filter(|(i, j)| final_map.board[*i][*j].has_been_visited())
        .collect::<Vec<_>>();

    // Then we iterate through that set, instead of the entire map
    let mut n_loops = 0;
    for (i, j) in possible_obstacles {
        let mut new_map = map.clone();
        new_map.board[i][j] = Square::Obstacle;
        new_map.run(false);
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
