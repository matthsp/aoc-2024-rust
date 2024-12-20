#[path = "utils/grid_utils.rs"] mod utils;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use itertools::Itertools;
use crate::utils::{parse_grid, Pos, Direction, get_value_from_grid, CARDINAL_DIRECTIONS};

advent_of_code::solution!(20);

fn parse(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let grid = parse_grid(input);
    let mut start: Pos = Pos::new(0, 0);
    let mut end: Pos = Pos::new(0, 0);

    for (i, l) in grid.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            match c {
                'S' => start = Pos::new(i as isize, j as isize),
                'E' => end = Pos::new(i as isize, j as isize),
                _ => {}
            }
        }
    }

    (grid, start, end)
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Pos,
    hack_activated: bool,
    hack_count: usize,
    path: Vec<(Pos, usize)>,
}

// Implement the Ord trait for the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).reverse()
    }
}

// Implement the PartialOrd trait for the priority queue
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<Vec<char>>, start: Pos, end: Pos) -> Option<(usize, Vec<(Pos, usize)>)> {
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start, hack_count: 0, hack_activated: false, path: vec![(start, 0)] });

    while let Some(State { cost, position, hack_count, hack_activated, path }) = heap.pop() {
        if position == end {
            return Some((cost, path));
        }

        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        for direction in CARDINAL_DIRECTIONS {
            let mut activate_hack = hack_activated.clone();
            let mut hack_new_count = hack_count.clone();
            let neighbor = position.neighbor(direction);
            let (x, y) = (
                neighbor.x as usize,
                neighbor.y as usize
            );

            let cell = get_value_from_grid(grid, x, y);
            if cell.is_none() {
                continue;
            }

            if *cell.unwrap() == '#' {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push((neighbor, cost + 1));
            let next = State { cost: cost + 1, position: neighbor, hack_count: hack_new_count, hack_activated: activate_hack, path: new_path };

            if next.cost < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                dist.insert(neighbor, next.cost);
                heap.push(next);
            }
        }
    }

    None
}

pub fn part_one_with_params(input: &str, threshold: usize) -> Option<usize> {
    let (grid, start, end) = parse(input);

    let path: Option<(usize, Vec<(Pos, usize)>)> = dijkstra(&grid, start, end);

    if path.is_none() {
        return None;
    }

    let (_, path) = path.unwrap();
    let mut result = 0;
    for (tuple) in path.iter().tuple_combinations() {
        let ((pos1, cost1), (pos2, cost2)) = tuple;
        let delta = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);
        if delta <= 2 && cost2.abs_diff(*cost1) >= delta + threshold {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_params(input, 100)
}

fn part_two_with_params(input: &str, threshold: usize) -> Option<usize> {
    let (grid, start, end) = parse(input);

    let path: Option<(usize, Vec<(Pos, usize)>)> = dijkstra(&grid, start, end);

    if path.is_none() {
        return None;
    }

    let (_, path) = path.unwrap();
    let mut result = 0;
    for (tuple) in path.iter().tuple_combinations() {
        let ((pos1, cost1), (pos2, cost2)) = tuple;
        let delta = pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y);
        if delta <= 20 && cost2.abs_diff(*cost1) >= delta + threshold {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_with_params(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_params(&advent_of_code::template::read_file("examples", DAY), 1);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_with_params(&advent_of_code::template::read_file("examples", DAY), 1);
        assert_eq!(result, Some(3081));
    }
}
