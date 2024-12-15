advent_of_code::solution!(15);

use std::io::{stdin, stdout, Write};

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
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
    // Part 2 only -- this means we have a few extra invariants to keep in mind,
    // as shown by the panic! calls, but saves on code duplication
    BoxRightHalf,
}

#[derive(Debug)]
struct Game {
    grid: Vec<Vec<Grid>>,
    robot: (usize, usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Part {
    One,
    Two,
}

impl Game {
    // If the box at `box_index` can be moved, this returns a Vec of indices of boxes that need to
    // be moved (including the one passed to this function). If it can't, returns an empty Vec.
    fn get_boxes_to_move_incl(
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
            // The process of collecting boxes that need to be moved is different for both parts,
            // so we have to split it up.
            Part::One => self.get_boxes_to_move_incl_p1(box_index, dir),
            Part::Two => self.get_boxes_to_move_incl_p2(box_index, dir),
        }
    }

    fn get_boxes_to_move_incl_p1(
        &mut self,
        this_index: (usize, usize),
        dir: Dir,
    ) -> Option<Vec<(usize, usize)>> {
        // Check what's in front of the box
        let next_index = get_next_index(this_index, &dir);
        match &self.grid[next_index.0][next_index.1] {
            // If it's empty, then the only thing that needs to be moved is this box itself
            Grid::Empty => Some(vec![this_index]),
            // If it's a wall, we can't move anything
            Grid::Wall => None,
            // If it's a box, we need to check if that box can be moved, so we call this function
            // recursively
            Grid::Box => match self.get_boxes_to_move_incl_p1(next_index, dir) {
                // If that returned None, it means the box couldn't be moved, so we
                // can't move this one either
                None => None,
                // Otherwise we can append ourselves to that list and return it
                Some(mut boxes_to_be_moved) => {
                    boxes_to_be_moved.push(this_index);
                    Some(boxes_to_be_moved)
                }
            },
            Grid::BoxRightHalf => panic!("BoxRightHalf found in part 1"),
        }
    }

    fn get_boxes_to_move_incl_p2(
        &mut self,
        this_index: (usize, usize),
        dir: Dir,
    ) -> Option<Vec<(usize, usize)>> {
        // The recursive behaviour of this function is exactly analogous to that of part 1
        // (get_boxes_to_move_incl_p1()), but with the added complexity that we need to
        // be more careful about where the boxes that can be moved are placed.
        match dir {
            Dir::W => {
                // For example, when moving west, we need to check whether next_index is a
                // BoxRightHalf, not a Box.
                let next_index = get_next_index(this_index, &Dir::W);
                match &self.grid[next_index.0][next_index.1] {
                    Grid::Empty => Some(vec![this_index]),
                    Grid::Wall => None,
                    Grid::Box => panic!("Shouldn't happen"),
                    Grid::BoxRightHalf => {
                        // Same code as for Part 1, except that we skip over the BoxRightHalf
                        // and check the Box to the left of it.
                        let next_next_index = get_next_index(next_index, &Dir::W);
                        match self.get_boxes_to_move_incl_p2(next_next_index, dir) {
                            None => None,
                            Some(mut next_boxes_to_be_moved) => {
                                next_boxes_to_be_moved.push(this_index);
                                Some(next_boxes_to_be_moved)
                            }
                        }
                    }
                }
            }
            Dir::E => {
                // When moving east, the next_index is guaranteed to be a BoxRightHalf, so we
                // need to skip over it.
                let next_next_index = get_next_index_multi(this_index, &[Dir::E, Dir::E]);
                match &self.grid[next_next_index.0][next_next_index.1] {
                    Grid::Empty => Some(vec![this_index]),
                    Grid::Wall => None,
                    // Otherwise, though, the behaviour is exactly the same as in Part 1.
                    Grid::Box => match self.get_boxes_to_move_incl_p2(next_next_index, dir) {
                        None => None,
                        Some(mut next_boxes_to_be_moved) => {
                            next_boxes_to_be_moved.push(this_index);
                            Some(next_boxes_to_be_moved)
                        }
                    },
                    Grid::BoxRightHalf => panic!("Shouldn't happen"),
                }
            }
            Dir::N | Dir::S => {
                // When moving north or south, it gets ugly.
                let next_index = get_next_index(this_index, &dir);
                let next_index_w = get_next_index(next_index, &Dir::W);
                let next_index_e = get_next_index(next_index, &Dir::E);
                // Say we are moving north. First, we check for walls above the box. The wall could
                // also be above the BoxRightHalf, so we need to check the northeast index as well.
                let possible_wall_indices = [next_index, next_index_e];
                if possible_wall_indices
                    .into_iter()
                    .any(|i| self.grid[i.0][i.1] == Grid::Wall)
                {
                    return None;
                }

                // If there are no walls, then we can check for boxes. We need to check three
                // possible indices for boxes.
                let possible_box_indices = [next_index_w, next_index, next_index_e];
                let mut boxes_to_be_moved = Vec::new();
                for i in possible_box_indices {
                    if self.grid[i.0][i.1] == Grid::Box {
                        match self.get_boxes_to_move_incl_p2(i, dir) {
                            // If _any_ of the boxes can't be moved, then the entire thing can't be
                            // moved, so we shortcircuit and return None.
                            None => return None,
                            // Otherwise, we collect all of them into a single Vec.
                            Some(next_boxes_to_be_moved) => {
                                boxes_to_be_moved.extend(next_boxes_to_be_moved);
                            }
                        }
                    }
                }
                // Don't forget to add ourselves!
                boxes_to_be_moved.push(this_index);
                Some(boxes_to_be_moved)
            }
        }
    }

    // Move a box together with all other boxes it would push. If any boxes were moved,
    // returns true. If no boxes were moved, returns false.
    fn move_box(&mut self, box_index: (usize, usize), dir: Dir, part: Part) -> bool {
        match self.get_boxes_to_move_incl(box_index, dir, part) {
            None => false,
            // Otherwise, move all of them
            Some(boxes_to_be_moved) => {
                let mut moved_boxes = Vec::new();

                for box_index in boxes_to_be_moved.into_iter() {
                    // Deduplicate. Duplicates can happen with e.g. diamond-shaped arrangements of
                    // boxes in part 2, where pushing one box pushes two boxes above it, which both
                    // push the same box.
                    if moved_boxes.contains(&box_index) {
                        continue;
                    }
                    moved_boxes.push(box_index);

                    // Actually push
                    let next_index = get_next_index(box_index, &dir);
                    self.grid[next_index.0][next_index.1] = Grid::Box;
                    self.grid[box_index.0][box_index.1] = Grid::Empty;

                    // For part 2, there's some special handling as we need to move both parts of
                    // the box.
                    if part == Part::Two {
                        // Move the BoxRightHalf along with the original Box
                        let new_east_index = get_next_index(next_index, &Dir::E);
                        self.grid[new_east_index.0][new_east_index.1] = Grid::BoxRightHalf;
                        // We need to clear the old BoxRightHalf, _unless_ we moved east, because
                        // in that case the Box will be moved into the BoxRightHalf's old position.
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

    // Move the robot in the given direction.
    fn move_robot(&mut self, dir: Dir, part: Part) {
        let next_index = get_next_index(self.robot, &dir);
        match &self.grid[next_index.0][next_index.1] {
            Grid::Empty => {
                self.robot = next_index;
            }
            Grid::Wall => {}
            Grid::Box => {
                if self.move_box(next_index, dir, part) {
                    self.robot = next_index;
                }
            }
            // Pushing against the right half is the same as pushing against the left half,
            // so we can reuse the function
            Grid::BoxRightHalf => {
                let box_left_half_index = get_next_index(next_index, &Dir::W);
                if self.move_box(box_left_half_index, dir, part) {
                    self.robot = next_index;
                }
            }
        }
    }
}

impl Game {
    fn _pretty_print(&self, part: Part) {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if (i, j) == self.robot {
                    print!("@");
                } else {
                    match cell {
                        Grid::Empty => print!("."),
                        Grid::Wall => print!("#"),
                        Grid::Box => match part {
                            Part::One => print!("O"),
                            Part::Two => print!("["),
                        },
                        Grid::BoxRightHalf => print!("]"),
                    }
                }
            }
            println!();
        }
    }

    fn sum_gps(&self) -> u32 {
        let mut gps = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell == &Grid::Box {
                    gps += (100 * i) + j;
                }
            }
        }
        gps as u32
    }
}

fn parse_input(input: &str, part: Part) -> (Game, Vec<Dir>) {
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
                        if part == Part::Two {
                            row.push(Grid::Empty);
                        }
                    }
                    '@' => {
                        row.push(Grid::Empty);
                        match part {
                            Part::One => robot = Some((i, j)),
                            Part::Two => {
                                row.push(Grid::Empty);
                                robot = Some((i, 2 * j));
                            }
                        }
                    }
                    '#' => {
                        row.push(Grid::Wall);
                        if part == Part::Two {
                            row.push(Grid::Wall);
                        }
                    }
                    'O' => {
                        row.push(Grid::Box);
                        if part == Part::Two {
                            row.push(Grid::BoxRightHalf);
                        }
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

// Play the game yourself! Interactively! With vim bindings!
fn _play_game(mut game: Game, part: Part) {
    loop {
        game._pretty_print(part);
        print!("hjkl/q: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        let n_bytes = stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "h" => game.move_robot(Dir::W, part),
            "j" => game.move_robot(Dir::S, part),
            "k" => game.move_robot(Dir::N, part),
            "l" => game.move_robot(Dir::E, part),
            "q" => break,
            "" => {
                if n_bytes == 0 {
                    break; // eof
                }
            }
            _ => continue,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const PART: Part = Part::One;
    let (mut game, moves) = parse_input(input, PART);
    moves.into_iter().for_each(|dir| game.move_robot(dir, PART));
    Some(game.sum_gps())
}

pub fn part_two(input: &str) -> Option<u32> {
    const PART: Part = Part::Two;
    let (mut game, moves) = parse_input(input, PART);
    moves.into_iter().for_each(|dir| game.move_robot(dir, PART));
    Some(game.sum_gps())
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
