advent_of_code::solution!(12);

use itertools::iproduct;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}


fn get_next_perimeter(
    peri: Perimeter,
    upwards: bool,
    nrows: usize,
    ncols: usize,
) -> Option<Perimeter> {
    match peri.direction {
        // A wall on the north or south edge means we need to go leftwards or rightwards
        Direction::N | Direction::S => {
            if upwards {
                if peri.point.j < ncols - 1 {
                    Some(Perimeter {
                        point: Point {
                            i: peri.point.i,
                            j: peri.point.j + 1,
                        },
                        direction: peri.direction,
                    })
                } else {
                    None
                }
            }
            else {
                if peri.point.j > 0 {
                    Some(Perimeter {
                        point: Point {
                            i: peri.point.i,
                            j: peri.point.j - 1,
                        },
                        direction: peri.direction,
                    })
                } else {
                    None
                }
            }
        },
        // A wall on the east or west edge means we need to go upwards or downwards
        Direction::E | Direction::W => {
            if upwards {
                if peri.point.i < nrows - 1 {
                    Some(Perimeter {
                        point: Point {
                            i: peri.point.i + 1,
                            j: peri.point.j,
                        },
                        direction: peri.direction,
                    })
                } else {
                    None
                }
            }
            else {
                if peri.point.i > 0 {
                    Some(Perimeter {
                        point: Point {
                            i: peri.point.i - 1,
                            j: peri.point.j,
                        },
                        direction: peri.direction,
                    })
                } else {
                    None
                }
            }
        },
    }
}
struct Perimeter {
    point: Point,
    direction: Direction,
}

struct Region {
    plant_type: char,
    points: Vec<Point>,
    area: u32,
    perimeter: Vec<Perimeter>,
}

// TODO: rename to i, j, nrows, ncols and fix off by 1 error
fn get_adjacents_and_inbounds(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(Direction, Option<Point>)> {
    vec![
        (
            Direction::N,
            if x > 0 {
                Some(Point { i: x - 1, j: y })
            } else {
                None
            },
        ),
        (
            Direction::S,
            if x < max_x {
                Some(Point { i: x + 1, j: y })
            } else {
                None
            },
        ),
        (
            Direction::E,
            if y < max_y {
                Some(Point { i: x, j: y + 1 })
            } else {
                None
            },
        ),
        (
            Direction::W,
            if y > 0 {
                Some(Point { i: x, j: y - 1 })
            } else {
                None
            },
        ),
    ]
}

fn to_grid(input: &str) -> Vec<Vec<(char, bool)>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| (c, false)).collect())
        .collect()
}

// Splits off the first region it can find, otherwise if the whole grid has already been
// covered returns None
fn split_off_region(grid: &mut [Vec<(char, bool)>]) -> Option<Region> {
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let index = iproduct!(0..n_rows, 0..n_cols).find(|(x, y)| !grid[*x][*y].1);
    match index {
        // Whole grid has already been traversed
        None => None,
        // Find the region starting at point (i, j)
        Some((i, j)) => {
            let mut region = Region {
                plant_type: grid[i][j].0,
                points: Vec::new(),
                area: 0,
                perimeter: Vec::new(),
            };
            let mut points_in_region = vec![Point { i, j }];

            // For each points in region
            while let Some(Point { i, j }) = points_in_region.pop() {
                // If it has already been visited, skip it. This can happen if we visit a square
                // that is adjacent to two other squares
                if grid[i][j].1 {
                    continue;
                }
                // Set it to visited
                grid[i][j].1 = true;
                // Add it to the region's points
                region.points.push(Point { i, j });
                // Increment the region's area
                region.area += 1;
                // Iterate over the adjacent tiles
                let adjacent_indices_and_inbounds =
                    get_adjacents_and_inbounds(i, j, n_rows - 1, n_cols - 1);
                for (dir, ind) in adjacent_indices_and_inbounds {
                    match ind {
                        // Adjacent tile is out of bounds, just add a perimeter
                        None => region.perimeter.push(Perimeter {
                            point: Point { i, j },
                            direction: dir,
                        }),
                        Some(Point { i: new_i, j: new_j }) => {
                            // If it's in bounds, add it to the list of points we need to traverse
                            if grid[new_i][new_j].0 == region.plant_type {
                                points_in_region.push(Point { i: new_i, j: new_j });
                            } else {
                                // If not then we need to add a perimeter on that side
                                region.perimeter.push(Perimeter {
                                    point: Point { i, j },
                                    direction: dir,
                                });
                            }
                        }
                    }
                }
            }

            // When we exit the loop, the region has been fully defined
            Some(region)
        }
    }
}

fn extract_regions(mut grid: Vec<Vec<(char, bool)>>) -> Vec<Region> {
    let mut regions = Vec::new();
    while let Some(region) = split_off_region(&mut grid) {
        regions.push(region);
    }
    regions
}

fn get_n_sides(mut region: Region, nrows: usize, ncols: usize) -> u32 {
    let mut n_sides = 0;
    // If there are still points, it means we found a new side
    while let Some(peri) = region.perimeter.pop() {
        n_sides += 1;
        // Search upwards and downwards and remove all points that are part of the same side
        match peri.direction {
            Direction::N | Direction::S => {
                let mut j_up = peri.point.j + 1;
                while j_up < ncols {
                    let p = region.perimeter.iter().position(|p| {
                        p.direction == peri.direction
                            && p.point.i == peri.point.i
                            && p.point.j == j_up
                    });
                    match p {
                        Some(p) => {
                            region.perimeter.remove(p);
                        }
                        None => break,
                    }
                    j_up += 1;
                }
                if peri.point.j > 0 {
                    let mut j_down = peri.point.j - 1;
                    loop {
                        let p = region.perimeter.iter().position(|p| {
                            p.direction == peri.direction
                                && p.point.i == peri.point.i
                                && p.point.j == j_down
                        });
                        match p {
                            Some(p) => {
                                region.perimeter.remove(p);
                            }
                            None => break,
                        }
                        if j_down == 0 {
                            break;
                        }
                        j_down -= 1;
                    }
                }
            }
            Direction::E | Direction::W => {
                let mut i_up = peri.point.i + 1;
                while i_up < nrows {
                    let p = region.perimeter.iter().position(|p| {
                        p.direction == peri.direction
                            && p.point.i == i_up
                            && p.point.j == peri.point.j
                    });
                    match p {
                        Some(p) => {
                            region.perimeter.remove(p);
                        }
                        None => break,
                    }
                    i_up += 1;
                }
                if peri.point.i > 0 {
                    let mut i_down = peri.point.i - 1;
                    loop {
                        let p = region.perimeter.iter().position(|p| {
                            p.direction == peri.direction
                                && p.point.i == i_down
                                && p.point.j == peri.point.j
                        });
                        match p {
                            Some(p) => {
                                region.perimeter.remove(p);
                            }
                            None => break,
                        }
                        if i_down == 0 {
                            break;
                        }
                        i_down -= 1;
                    }
                }
            }
        }
    }
    n_sides
}

pub fn part_one(input: &str) -> Option<u32> {
    let regions = extract_regions(to_grid(input));
    let price = regions.into_iter().fold(0, |acc, region| {
        acc + (region.area * region.perimeter.len() as u32)
    });
    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = to_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let regions = extract_regions(grid);
    let price = regions.into_iter().fold(0, |acc, region| {
        acc + (region.area * get_n_sides(region, nrows, ncols))
    });
    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
