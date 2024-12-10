use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(9);

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum BlockType {
    File{id: u32},
    FreeSpace,
}

#[derive(Clone)]
struct Block {
    size: u32,
    block_type: BlockType,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = VecDeque::new();
    let mut id = 0;
    let mut format = BlockType::File{id};
    for char in input.trim_end().chars() {
        map.push_back(Block {
            size: char.to_digit(10).expect("Fits into u32"),
            block_type: format,
        });
        format = match format {
            BlockType::File{..} => BlockType::FreeSpace,
            BlockType::FreeSpace => {
                id += 1;
                BlockType::File{id}
            },
        };
    };

    let mut new_map = vec![];
    'outer: loop {
        if map.is_empty() {
            break;
        };
        let next_block = map.pop_front().expect("Unless input is empty this should be fine");
        match next_block.block_type {
            BlockType::File{..} => new_map.push(next_block),
            BlockType::FreeSpace => {
                let end_block;
                loop {
                    match map.pop_back() {
                        Some(block) if block.block_type == BlockType::FreeSpace => continue,
                        Some(block) => {
                            end_block = block;
                            break;
                        },
                        None => break 'outer,  // If there is no end block then we are done
                    };
                };
                if next_block.size > end_block.size {
                    // fit the end block into the empty space in the next block
                    // and push the rest of the empty space back onto the beginning of the q
                    let new_size = next_block.size - end_block.size;
                    new_map.push(Block {
                        size: end_block.size,
                        block_type: end_block.block_type,
                    });
                    map.push_front(Block {
                        size: new_size,
                        block_type: BlockType::FreeSpace,
                    });
                }
                else if next_block.size < end_block.size {
                    // fit as much of the end block into the empty space in the next block
                    // and push the rest of the end block back onto the end of the queue
                    let new_size = end_block.size - next_block.size;
                    new_map.push(Block {
                        size: next_block.size,
                        block_type: end_block.block_type,
                    });
                    map.push_back(Block {
                        size: new_size,
                        block_type: end_block.block_type,
                    });
                }
                else { // sizes are exactly the same
                    new_map.push(end_block);
                }
            }
        }

    }

    let mut total: u64 = 0;
    let mut start_pos = 0;
    for block in new_map {
        let id: u64 = match block.block_type {
            BlockType::File{id} => id,
            BlockType::FreeSpace => panic!("Should not have any free space blocks"),
        }.try_into().expect("Fits into u64");
        let block_size: u64 = block.size.try_into().expect("Fits into u64");
        total += (start_pos..(start_pos + block_size)).sum::<u64>() * id;
        start_pos += block_size;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = VecDeque::new();
    let mut id = 0;
    let mut format = BlockType::File{id};
    let mut files = vec![];
    for char in input.trim_end().chars() {
        let char_int = char.to_digit(10).expect("Fits into u32");
        map.push_back(Block {
            size: char_int,
            block_type: format,
        });
        format = match format {
            BlockType::File{..} => {
                files.push(Block {
                    size: char_int,
                    block_type: format,
                });

                BlockType::FreeSpace
            },
            BlockType::FreeSpace => {
                id += 1;
                BlockType::File{id}
            },
        };
    };

    let mut used_files = HashSet::new();
    let mut new_map = vec![];

    loop {
        if map.is_empty() {
            break;
        };
        let next_block = map.pop_front().expect("Unless input is empty this should be fine");
        match next_block.block_type {
            BlockType::File{id} if used_files.contains(&id) =>
                new_map.push(Block {
                    size: next_block.size,
                    block_type: BlockType::FreeSpace,
                }),
            BlockType::File{id} => {
                used_files.insert(id);
                new_map.push(next_block);
            },
            BlockType::FreeSpace => {
                let mut matched_file = false;
                for block in files.iter().rev() {
                    let id = match block.block_type {
                        BlockType::File { id } => id,
                        BlockType::FreeSpace => panic!("Should not have any free space blocks"),
                    };
                    if used_files.contains(&id) {
                        continue;
                    }
                    if block.size > next_block.size {
                        continue;
                    }
                    matched_file = true;
                    used_files.insert(id);
                    new_map.push((*block).clone());
                    let empty_block_size = next_block.size - block.size;
                    if empty_block_size > 0 {
                        map.push_front(Block {
                            size: empty_block_size,
                            block_type: BlockType::FreeSpace,
                        });
                    };
                    break;
                }
                if !matched_file {
                    // no files could fit into the empty space
                    // so we keep the empty space block
                    new_map.push(next_block);
                }
            }
        }
    }
    let mut total: u64 = 0;
    let mut start_pos = 0;
    for block in new_map {
        let block_size: u64 = block.size.try_into().expect("Fits into u64");
        if let BlockType::File{id} = block.block_type {
            let file_id: u64 = id.try_into().expect("Fits into u64");
            total += (start_pos..(start_pos + block_size)).sum::<u64>() * file_id;
        }
        start_pos += block_size;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2858));
    }
    #[test]
    fn test_part_two_ensure_file_are_not_moved_rightwards() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(132));
    }
}
