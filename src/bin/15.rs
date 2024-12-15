advent_of_code::solution!(15);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn get_next_index(cur_index: (usize, usize), dir: &Dir) -> (usize, usize) {
    // No need for bounds check because there is an edge of walls which will never be moved / moved
    // into
    match dir {
        Dir::N => (cur_index.0 - 1, cur_index.1),
        Dir::E => (cur_index.0, cur_index.1 + 1),
        Dir::S => (cur_index.0 + 1, cur_index.1),
        Dir::W => (cur_index.0, cur_index.1 - 1),
    }
}

fn get_next_index_multi(cur_index: (usize, usize), dirs: &[Dir]) -> (usize, usize) {
    dirs.iter().fold(cur_index, get_next_index)
}

#[derive(Debug, PartialEq, Eq)]
enum Grid {
    Empty,
    Wall,
    Box,
    BoxRightHalf, // Part 2 only
}

#[derive(Debug)]
struct Game {
    grid: Vec<Vec<Grid>>,
    robot: (usize, usize),
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Part {
    One,
    Two,
}

impl Game {
    // If the box can be moved, returns a Vec of indices of boxes that need to be moved (including
    // the one passed to this function). If it can't, returns an empty Vec. The Vec is in the order
    // of boxes that need to be moved, starting from the one furthest from the robot.
    fn get_boxes_to_move(
        &mut self,
        box_index: (usize, usize),
        dir: Dir,
        part: Part,
    ) -> Option<Vec<(usize, usize)>> {
        // Note that box_index should only contain a Box, NOT a BoxRightHalf
        match &self.grid[box_index.0][box_index.1] {
            Grid::Box => {}
            _ => panic!("get_boxes_to_move called with non-box index"),
        }
        match part {
            Part::One => self.get_boxes_to_move_p1(box_index, dir),
            Part::Two => self.get_boxes_to_move_p2(box_index, dir),
        }
    }

    fn get_boxes_to_move_p1(
        &mut self,
        box_index: (usize, usize),
        dir: Dir,
    ) -> Option<Vec<(usize, usize)>> {
        let next_index = get_next_index(box_index, &dir);
        match &self.grid[next_index.0][next_index.1] {
            Grid::Empty => Some(vec![box_index]),
            Grid::Wall => None,
            Grid::Box => match self.get_boxes_to_move_p1(next_index, dir.clone()) {
                None => None,
                Some(mut next_boxes_to_be_moved) => {
                    if let Grid::Box = self.grid[box_index.0][box_index.1] {
                        next_boxes_to_be_moved.push(box_index);
                    }
                    Some(next_boxes_to_be_moved)
                }
            },
            Grid::BoxRightHalf => panic!("BoxRightHalf found in part 1"),
        }
    }

    fn get_boxes_to_move_p2(
        &mut self,
        box_index: (usize, usize),
        dir: Dir,
    ) -> Option<Vec<(usize, usize)>> {
        // Need to go case by case... ugh
        match dir {
            Dir::W => {
                let next_index = get_next_index(box_index, &Dir::W);
                match &self.grid[next_index.0][next_index.1] {
                    Grid::Empty => Some(vec![box_index]),
                    Grid::Wall => None,
                    Grid::Box => panic!("Shouldn't happen"),
                    Grid::BoxRightHalf => {
                        let next_next_index = get_next_index(next_index, &Dir::W);
                        match self.get_boxes_to_move_p2(next_next_index, dir) {
                            None => None,
                            Some(mut next_boxes_to_be_moved) => {
                                next_boxes_to_be_moved.push(box_index);
                                Some(next_boxes_to_be_moved)
                            }
                        }
                    }
                }
            }
            Dir::E => {
                let next_next_index = get_next_index_multi(box_index, &[Dir::E, Dir::E]);
                match &self.grid[next_next_index.0][next_next_index.1] {
                    Grid::Empty => Some(vec![box_index]),
                    Grid::Wall => None,
                    Grid::Box => match self.get_boxes_to_move_p2(next_next_index, dir) {
                        None => None,
                        Some(mut next_boxes_to_be_moved) => {
                            next_boxes_to_be_moved.push(box_index);
                            Some(next_boxes_to_be_moved)
                        }
                    },
                    Grid::BoxRightHalf => panic!("Shouldn't happen"),
                }
            }
            Dir::N | Dir::S => {
                let next_index = get_next_index(box_index, &dir);
                let next_index_w = get_next_index(next_index, &Dir::W);
                let next_index_e = get_next_index(next_index, &Dir::E);
                // First check for walls above
                let possible_wall_indices = [next_index, next_index_e];
                if possible_wall_indices
                    .into_iter()
                    .any(|i| self.grid[i.0][i.1] == Grid::Wall)
                {
                    return None;
                }

                // If none, then we can check for boxes
                let possible_box_indices = [next_index_w, next_index, next_index_e];
                let mut boxes_to_be_moved = Vec::new();
                for i in possible_box_indices {
                    if self.grid[i.0][i.1] == Grid::Box {
                        match self.get_boxes_to_move_p2(i, dir.clone()) {
                            // That box can't be moved, so shortcircuit
                            None => return None,
                            Some(next_boxes_to_be_moved) => {
                                boxes_to_be_moved.extend(next_boxes_to_be_moved);
                            }
                        }
                    }
                }
                boxes_to_be_moved.push(box_index);
                Some(boxes_to_be_moved)
            }
        }
    }

    // Actually moves the box, returns true if the box was moved, false if not
    fn move_box(&mut self, box_index: (usize, usize), dir: Dir, part: Part) -> bool {
        match self.get_boxes_to_move(box_index, dir.clone(), part.clone()) {
            None => false,
            // Otherwise, move all of them
            Some(boxes_to_be_moved) => {
                let mut moved_boxes = Vec::new();

                for box_index in boxes_to_be_moved.into_iter() {
                    // Deduplicate
                    if moved_boxes.contains(&box_index) {
                        continue;
                    }
                    moved_boxes.push(box_index);

                    // Actually push
                    let next_index = get_next_index(box_index, &dir);
                    self.grid[next_index.0][next_index.1] = Grid::Box;
                    self.grid[box_index.0][box_index.1] = Grid::Empty;
                    // Update the index to the right of the new box (part 2 only)
                    if part == Part::Two {
                        let new_east_index = get_next_index(next_index, &Dir::E);
                        self.grid[new_east_index.0][new_east_index.1] = Grid::BoxRightHalf;
                        if dir != Dir::E {
                            let old_east_index = get_next_index(box_index, &Dir::E);
                            self.grid[old_east_index.0][old_east_index.1] = Grid::Empty;
                        }
                    }
                }
                true
            }
        }
    }

    fn move_robot(&mut self, dir: &Dir, part: Part) {
        let next_index = get_next_index(self.robot, dir);
        match &self.grid[next_index.0][next_index.1] {
            Grid::Empty => {
                self.robot = next_index;
            }
            Grid::Wall => {}
            Grid::Box => {
                if self.move_box(next_index, dir.clone(), part) {
                    self.robot = next_index;
                }
            }
            // Pushing against the right half is the same as pushing against the left half,
            // so we can reuse the function
            Grid::BoxRightHalf => {
                let box_left_half_index = get_next_index(next_index, &Dir::W);
                if self.move_box(box_left_half_index, dir.clone(), part) {
                    self.robot = next_index;
                    // But we also need to make sure to clear the right half
                    self.grid[next_index.0][next_index.1] = Grid::Empty;
                }
            }
        }
    }
}

impl Game {
    fn _pretty_print(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if (i, j) == self.robot {
                    print!("@");
                } else {
                    match cell {
                        Grid::Empty => print!("."),
                        Grid::Wall => print!("#"),
                        Grid::Box => print!("O"),
                        Grid::BoxRightHalf => print!("}}"),
                    }
                }
            }
            println!();
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let Grid::Box = cell {
                    score += (100 * i) + j;
                }
            }
        }
        score as u32
    }
}

fn parse_input_p1(input: &str) -> (Game, Vec<Dir>) {
    if let Some((game_str, moves_str)) = input.split_once("\n\n") {
        // Parse game state
        let mut robot = None;
        let mut grid = Vec::new();
        for (i, line) in game_str.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Grid::Empty),
                    '@' => {
                        row.push(Grid::Empty);
                        robot = Some((i, j));
                    }
                    '#' => row.push(Grid::Wall),
                    'O' => row.push(Grid::Box),
                    _ => panic!("Invalid character in input: {}", c),
                }
            }
            grid.push(row);
        }
        let game = Game {
            grid,
            robot: robot.expect("No robot found"),
        };
        // Parse moves
        let moves = moves_str
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Dir::N),
                '>' => Some(Dir::E),
                'v' => Some(Dir::S),
                '<' => Some(Dir::W),
                _ => None, // Drop all other characters
            })
            .collect();
        return (game, moves);
    }
    panic!("Failed parse");
}

fn parse_input_p2(input: &str) -> (Game, Vec<Dir>) {
    if let Some((game_str, moves_str)) = input.split_once("\n\n") {
        // Parse game state
        let mut robot = None;
        let mut grid = Vec::new();
        for (i, line) in game_str.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        row.push(Grid::Empty);
                        row.push(Grid::Empty);
                    }
                    '@' => {
                        row.push(Grid::Empty);
                        row.push(Grid::Empty);
                        robot = Some((i, 2 * j));
                    }
                    '#' => {
                        row.push(Grid::Wall);
                        row.push(Grid::Wall);
                    }
                    'O' => {
                        row.push(Grid::Box);
                        row.push(Grid::BoxRightHalf);
                    }
                    _ => panic!("Invalid character in input: {}", c),
                }
            }
            grid.push(row);
        }
        let game = Game {
            grid,
            robot: robot.expect("No robot found"),
        };
        // Parse moves
        let moves = moves_str
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Dir::N),
                '>' => Some(Dir::E),
                'v' => Some(Dir::S),
                '<' => Some(Dir::W),
                _ => None, // Drop all other characters
            })
            .collect();
        return (game, moves);
    }
    panic!("Failed parse");
}

use std::io::{stdin, stdout, Write};

pub fn part_one(input: &str) -> Option<u32> {
    let (mut game, moves) = parse_input_p1(input);
    for dir in moves {
        game.move_robot(&dir, Part::One);
    }
    // game._pretty_print();
    Some(game.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut game, moves) = parse_input_p2(input);
    for dir in moves {
        game.move_robot(&dir, Part::Two);
    }
    // game._pretty_print();
    Some(game.score())

    // // Play the game yourself!
    // loop {
    //     game._pretty_print();
    //     print!("hjkl/q: ");
    //     stdout().flush().unwrap();
    //     let mut input = String::new();
    //     let n_bytes = stdin().read_line(&mut input).unwrap();
    //     match input.trim() {
    //         "h" => game.move_robot(&Dir::W, Part::Two),
    //         "j" => game.move_robot(&Dir::S, Part::Two),
    //         "k" => game.move_robot(&Dir::N, Part::Two),
    //         "l" => game.move_robot(&Dir::E, Part::Two),
    //         "q" => break,
    //         "" => {
    //             if n_bytes == 0 {
    //                 break; // eof
    //             }
    //         }
    //         _ => continue,
    //     }
    // }
    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
