advent_of_code::solution!(4);

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NW,
    Direction::W,
    Direction::SW,
    Direction::S,
    Direction::SE,
    Direction::E,
    Direction::NE,
];

// X[i][j] is the i-th row, j-th column, just like matrices
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_indices(matrix: &[Vec<char>], target_char: char) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == target_char {
                indices.push((i, j));
            }
        }
    }
    indices
}

fn get_forbidden_directions(
    n_rows: usize,
    n_cols: usize,
    first_index: (usize, usize),
    extent: usize,
) -> HashSet<Direction> {
    let mut forbidden_directions = HashSet::new();
    if first_index.0 < extent {
        forbidden_directions.insert(Direction::N);
        forbidden_directions.insert(Direction::NW);
        forbidden_directions.insert(Direction::NE);
    }
    if first_index.0 >= n_rows - extent {
        forbidden_directions.insert(Direction::S);
        forbidden_directions.insert(Direction::SW);
        forbidden_directions.insert(Direction::SE);
    }
    if first_index.1 < extent {
        forbidden_directions.insert(Direction::W);
        forbidden_directions.insert(Direction::NW);
        forbidden_directions.insert(Direction::SW);
    }
    if first_index.1 >= n_cols - extent {
        forbidden_directions.insert(Direction::E);
        forbidden_directions.insert(Direction::NE);
        forbidden_directions.insert(Direction::SE);
    }
    forbidden_directions
}

fn next_index(current_index: (usize, usize), direction: &Direction) -> (usize, usize) {
    let (row, col) = current_index;
    match direction {
        Direction::N => (row - 1, col),
        Direction::NW => (row - 1, col - 1),
        Direction::W => (row, col - 1),
        Direction::SW => (row + 1, col - 1),
        Direction::S => (row + 1, col),
        Direction::SE => (row + 1, col + 1),
        Direction::E => (row, col + 1),
        Direction::NE => (row - 1, col + 1),
    }
}

fn is_xmas(matrix: &[Vec<char>], x_index: (usize, usize), direction: &Direction) -> bool {
    let m_index = next_index(x_index, direction);
    if matrix[m_index.0][m_index.1] != 'M' {
        return false;
    }
    let a_index = next_index(m_index, direction);
    if matrix[a_index.0][a_index.1] != 'A' {
        return false;
    }
    let s_index = next_index(a_index, direction);
    if matrix[s_index.0][s_index.1] != 'S' {
        return false;
    }
    true
}

/// Around 5x slower(!)
// fn count_xmases_beginning_at(matrix: &[Vec<char>], x_index: (usize, usize)) -> u32 {
//     let forbidden_directions = get_forbidden_directions(matrix.len(), matrix[0].len(), x_index, 3);
//     HashSet::from(ALL_DIRECTIONS)
//         .difference(&forbidden_directions)
//         .collect::<Vec<&Direction>>()
//         .iter()
//         .fold(0, |acc, dir| acc + is_xmas(matrix, x_index, dir) as u32)
// }

fn count_xmases_beginning_at(matrix: &[Vec<char>], x_index: (usize, usize)) -> u32 {
    let forbidden_directions = get_forbidden_directions(matrix.len(), matrix[0].len(), x_index, 3);
    ALL_DIRECTIONS.iter().fold(0, |acc, dir| {
        acc + (!forbidden_directions.contains(dir) && is_xmas(matrix, x_index, dir)) as u32
    })
}

/// Brute force search :(
pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    Some(
        get_indices(&matrix, 'X')
            .into_iter()
            .fold(0, |acc, idx| acc + count_xmases_beginning_at(&matrix, idx)),
    )
}

fn is_crossmas(matrix: &[Vec<char>], a_index: (usize, usize)) -> bool {
    // Shortcircuit if out of bounds
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    if a_index.0 == 0 || a_index.0 == n_rows - 1 || a_index.1 == 0 || a_index.1 == n_cols - 1 {
        return false;
    }
    // Check characters
    let nw_index = next_index(a_index, &Direction::NW);
    let ne_index = next_index(a_index, &Direction::NE);
    let sw_index = next_index(a_index, &Direction::SW);
    let se_index = next_index(a_index, &Direction::SE);
    let nw_char = matrix[nw_index.0][nw_index.1];
    let ne_char = matrix[ne_index.0][ne_index.1];
    let sw_char = matrix[sw_index.0][sw_index.1];
    let se_char = matrix[se_index.0][se_index.1];
    (nw_char == 'M' && ne_char == 'M' && se_char == 'S' && sw_char == 'S')
        || (nw_char == 'S' && ne_char == 'M' && se_char == 'M' && sw_char == 'S')
        || (nw_char == 'S' && ne_char == 'S' && se_char == 'M' && sw_char == 'M')
        || (nw_char == 'M' && ne_char == 'S' && se_char == 'S' && sw_char == 'M')
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    Some(
        get_indices(&matrix, 'A')
            .into_iter()
            .fold(0, |acc, idx| acc + is_crossmas(&matrix, idx) as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
