use std::collections::HashMap;
use itertools::{Either, Itertools};

advent_of_code::solution!(25);

fn parse(input: &str) -> (HashMap<usize, (bool, Vec<usize>)>, usize) {
    let mut result: HashMap<usize, (bool, Vec<usize>)> = HashMap::new();
    let mut max_height = 0;

    input.split("\n\n").enumerate().for_each(
        |(index, i)| {
            let grid: Vec<Vec<char>> = i.lines().map(|l| l.chars().collect()).collect();
            let height = grid.len() - 1;
            let width = grid[0].len();
            let mut heights: Vec<usize> = Vec::new();

            max_height = height - 1;

            for j in 0..width {
               let mut column_height = 0;

                // Do not count first and last row
                for i in 1..height {
                    if grid[i][j] == '#' {
                        column_height += 1;
                    }
                }

                heights.push(column_height);
            }

            result.insert(index, (grid[0][0] == '#', heights));
        }
    );

    (result, max_height)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (locks_and_keys, max_height) = parse(input);

    let (locks, keys): (Vec<_>, Vec<_>) = locks_and_keys.values().partition_map(
        |l| match l.0 {
            true => Either::Left(&l.1),
            false => Either::Right(&l.1),
        }
    );

    let result: Vec<(&Vec<usize>, usize)> = locks.iter().map(
        |lock| {
            (lock.clone(), keys.iter().filter(|key| {
                for (ha, hb) in key.iter().zip(lock.iter()) {
                    if ha + hb > max_height {
                        return false;
                    }
                }
                true
            }).count())
        }
    ).collect();

    Some(result.iter().map(|(_, count)| *count).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
