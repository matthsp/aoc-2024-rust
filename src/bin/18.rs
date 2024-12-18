#[path = "utils/grid_utils.rs"] mod utils;

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use crate::utils::{get_value_from_grid, Direction, Pos};

advent_of_code::solution!(18);

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Pos,
    prev_direction: Option<Direction>,
    path: Vec<Pos>,
}


fn parse(input: &str, size: usize, bytes: usize) -> (Vec<Pos>, Vec<Vec<char>>) {
    let mut count = 0;
    let mut grid = vec![vec!['.'; size + 1]; size + 1];
    let positions_left = input.lines().filter_map(
        |l| {
            let coord: Vec<isize> = l.split(',').map(|c| c.parse::<isize>().unwrap()).collect();
            let x = coord[0];
            let y = coord[1];
            if count < bytes {
                grid[y as usize][x as usize] = '#';
                count += 1;
                return None;
            }
            Some(Pos::new(x, y))
        }
    ).collect();

    (positions_left, grid)
}

fn bfs(start: Pos, goal: Pos, grid: &Vec<Vec<char>>) -> Option<Vec<Pos>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut parent = vec![vec![None; grid[0].len()]; grid.len()];

    queue.push_back(start);
    visited[start.y as usize][start.x as usize] = true;

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut pos = Some(current);
            while let Some(p) = pos {
                path.push(p);
                pos = parent[p.y as usize][p.x as usize];
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in current.neighbors(&[Direction::East, Direction::South, Direction::West, Direction::North]) {
            let val = get_value_from_grid(&grid, neighbor.x as usize, neighbor.y as usize);
            if let Some(cell) = val {
                if !visited[neighbor.y as usize][neighbor.x as usize] && *cell != '#' {
                    visited[neighbor.y as usize][neighbor.x as usize] = true;
                    parent[neighbor.y as usize][neighbor.x as usize] = Some(current);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    None
}

pub fn part_one_with_params(input: &str, size: usize, bytes: usize) -> Option<usize> {
    let (positions_left, grid) = parse(input, size, bytes);

    let start_pos = Pos::new(0, 0);
    let end_pos = Pos::new(size as isize, size as isize);

    let result = bfs(start_pos, end_pos, &grid);
    if result.is_some() {
        return Some(result.unwrap().len() - 1)
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_params(input, 70, 1024)
}

fn part_two_with_params(input: &str, size: usize, bytes: usize) -> Option<String> {
    let (positions, mut grid) = parse(input, size, bytes);
    let start_pos = Pos::new(0, 0);
    let end_pos = Pos::new(size as isize, size as isize);

    for pos in positions {
        grid[pos.y as usize][pos.x as usize] = '#';

        let result = bfs(start_pos, end_pos, &grid);
        if result.is_none() {
            return Some(format!("{},{}", pos.x, pos.y))
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_with_params(input, 70, 2038)
}

#[cfg(test)]
mod tests {
    use tinyjson::JsonValue::String;
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_params(&advent_of_code::template::read_file("examples", DAY), 6, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_with_params(&advent_of_code::template::read_file("examples", DAY), 6, 20);
        assert_eq!(result, Some("6,1".to_string()));
    }
}
