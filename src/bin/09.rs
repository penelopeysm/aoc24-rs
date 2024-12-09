advent_of_code::solution!(9);

use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn _print_blocks(blocks: &[Option<u32>]) {
    for bl in blocks {
        match bl {
            None => print!("."),
            Some(ctx) => print!("{}", ctx),
        }
    }
    println!();
}

struct BlockBlocks {
    starting_index: usize,
    n_blocks: usize,
    block_contents: Option<u32>,
}

fn flatten(bbs: &[BlockBlocks]) -> Vec<Option<u32>> {
    bbs.iter()
        .flat_map(|bb| vec![bb.block_contents; bb.n_blocks])
        .collect()
}

fn parse_input(input: &str) -> Vec<BlockBlocks> {
    let mut counter = 0;
    let mut cur_index = 0;
    let mut bbs: Vec<BlockBlocks> = vec![];
    let mut is_file = true;
    for char in input.trim().chars() {
        let maybe_n = char.to_digit(10);
        match maybe_n {
            None => break,
            Some(n) => {
                if is_file {
                    bbs.push(BlockBlocks {
                        starting_index: cur_index,
                        n_blocks: n as usize,
                        block_contents: Some(counter),
                    });
                    cur_index += n as usize;
                    counter += 1;
                    is_file = false;
                } else {
                    bbs.push(BlockBlocks {
                        starting_index: cur_index,
                        n_blocks: n as usize,
                        block_contents: None,
                    });
                    cur_index += n as usize;
                    is_file = true;
                }
            }
        }
    }
    bbs
}

fn move_blocks_1(blocks: &mut [Option<u32>]) {
    // Pointers to start and end
    let mut i = 0;
    let mut j = blocks.len() - 1;

    for _ in 0..blocks.len() {
        if i >= j {
            break;
        }
        match blocks[i] {
            Some(_) => {
                i += 1;
            }
            None => {
                while blocks[j].is_none() {
                    j -= 1;
                    if i >= j {
                        break;
                    }
                }
                match blocks[j] {
                    None => break, // Shouldn't happen
                    Some(c) => {
                        blocks[i] = Some(c);
                        blocks[j] = None;
                        i += 1;
                        j -= 1;
                    }
                }
            }
        }
    }
}

fn move_blocks_2(bbs: &[BlockBlocks]) -> Vec<Option<u32>> {
    // Construct hashmap of empty blocks
    // keys = size of empty block
    // values = starting indices of empty blocks of that size
    let mut empty_map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for bb in bbs.iter() {
        if bb.block_contents.is_none() {
            let n_blocks = bb.n_blocks;
            empty_map
                .entry(n_blocks)
                .or_default()
                .insert(bb.starting_index);
        }
    }

    // returns index + the number of empty spaces starting at that index
    fn find_first_empty_space(
        map: &BTreeMap<usize, BTreeSet<usize>>,
        n_blocks: usize,
        max_index: usize,
    ) -> Option<(usize, usize)> {
        let mut all_possibilities = map
            .iter()
            .filter_map(|(size, indices)| {
                if *size < n_blocks {
                    None
                } else {
                    match indices.first() {
                        None => None,
                        Some(index) => {
                            if *index > max_index {
                                None
                            } else {
                                Some((*index, *size))
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        all_possibilities.sort();
        all_possibilities.first().copied()
    }

    // Construct flattened vector
    let mut blocks = flatten(bbs);

    // Move blocks from the right-hand side
    for bb in bbs.iter().rev() {
        if let Some(value) = bb.block_contents {
            if let Some((target_index, target_size)) =
                find_first_empty_space(&empty_map, bb.n_blocks, bb.starting_index)
            {
                // println!("Moving block {} to {}", value, target_index);
                empty_map
                    .get_mut(&target_size)
                    .unwrap()
                    .remove(&target_index);
                // calculate new starting index
                if target_size > bb.n_blocks {
                    let new_empty_length = target_size - bb.n_blocks;
                    let new_starting_index = target_index + bb.n_blocks;
                    empty_map
                        .entry(new_empty_length)
                        .or_default()
                        .insert(new_starting_index);
                }
                // modify flattened blocks
                for i in 0..bb.n_blocks {
                    blocks[bb.starting_index + i] = None;
                    blocks[target_index + i] = Some(value);
                }
            }
        }
    }

    blocks
}

fn get_checksum(blocks: Vec<Option<u32>>) -> u64 {
    let mut checksum = 0;
    for (i, bl) in blocks.into_iter().enumerate() {
        match bl {
            None => continue,
            Some(c) => {
                checksum += i as u64 * c as u64;
            }
        }
    }
    checksum
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = flatten(&parse_input(input));
    move_blocks_1(&mut blocks);
    // print_blocks(&blocks);
    Some(get_checksum(blocks))
}

pub fn part_two(input: &str) -> Option<u64> {
    let blockblocks = parse_input(input);
    let blocks = move_blocks_2(&blockblocks);
    Some(get_checksum(blocks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
