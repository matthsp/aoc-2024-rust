advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<u32> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[derive(Debug)]
struct Block {
    file_id: usize,
    index: usize,
}

fn get_blocks(index: usize, count: &u32, file_id: usize) -> Vec<Block> {
    (index.. *count as usize + index)
        .map(|i| Block { file_id, index: i })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let spaces = parse(input);

    let mut indexes: usize = 0;
    let mut free_spaces: Vec<usize> = Vec::new();
    let mut file_id = 0;

    let mut blocks: Vec<Block> = Vec::new();

    spaces.iter().enumerate().for_each(
        |(index, count)| {
            let new_vec = get_blocks(indexes, count, file_id);

            match index % 2 {
                1 => {
                    let free_indexes: Vec<usize> = new_vec.iter().map(|b| b.index).collect();
                    free_spaces.extend(free_indexes);
                    indexes += *count as usize;
                },
                0 => {
                    blocks.extend(new_vec);
                    indexes += *count as usize;
                    file_id += 1;
                },
                _ => unimplemented!()
            }
        }
    );

    free_spaces.sort_by(|a, b| b.cmp(a));

    blocks.iter_mut().rev().for_each(
        |b| {
            if let Some(free_block) = free_spaces.last() {
                if *free_block < b.index {
                    b.index = *free_block;
                    free_spaces.pop();
                }
            }
        }
    );

    Some(blocks.iter()
        .map(|b| {
            return b.index * b.file_id;
        }).sum())
}

#[derive(Debug)]
struct BlockPartTwo {
    first_index: usize,
    indexes: Vec<usize>,
    file_id: usize,
}

fn get_block_part_two(index: usize, count: &u32, file_id: usize) -> BlockPartTwo {
    let indexes = (index.. *count as usize + index).collect();
    BlockPartTwo { first_index: index, indexes, file_id }
}

pub fn part_two(input: &str) -> Option<usize> {
    let spaces = parse(input);

    let mut indexes: usize = 0;
    let mut free_spaces: Vec<Vec<usize>> = Vec::new();
    let mut file_id = 0;

    let mut blocks: Vec<BlockPartTwo> = Vec::new();

    spaces.iter().enumerate().for_each(
        |(index, count)| {
            let block = get_block_part_two(indexes, count, file_id);

            match index % 2 {
                1 => {
                    let mut free_indexes: Vec<usize> = block.indexes;
                    // sort to ease popping out element
                    free_indexes.sort_by(|a, b| b.cmp(a));
                    free_spaces.push(free_indexes);
                    indexes += *count as usize;
                },
                0 => {
                    blocks.push(block);
                    indexes += *count as usize;
                    file_id += 1;
                },
                _ => unimplemented!()
            }
        }
    );

    blocks.iter_mut().rev().for_each(
        |b| {
            for free_block in free_spaces.iter_mut() {
                if free_block.len() >= b.indexes.len() {
                    if free_block.last().unwrap() > &b.first_index {
                        continue;
                    }

                    // extract free blocks
                    let mut new_indexes: Vec<usize> = Vec::new();

                    for _i in 0..b.indexes.len() {
                        new_indexes.push(free_block.pop().unwrap());
                    }
                    b.first_index = new_indexes[0];
                    b.indexes = new_indexes;
                    break;
                }
            }
        }
    );

    blocks.sort_by(|a, b| a.first_index.cmp(&b.first_index));

    let mut result: usize = 0;
    blocks.iter()
        .flat_map(|b| {
            let blocks: Vec<Block> = b.indexes.iter().map(|i| Block { index: *i, file_id: b.file_id }).collect();
            return blocks;
        })
        .for_each(|b| {
            result += b.index * b.file_id;
        });

    Some(result)
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
