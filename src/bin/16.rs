use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
#[path = "utils/grid_utils.rs"] mod utils;
use crate::utils::{parse_grid, Pos, Direction};

advent_of_code::solution!(16);

fn parse(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let grid = parse_grid(input);
    let mut start: Pos = Pos::new(0, 0);
    let mut end: Pos = Pos::new(0, 0);

    for (i, l) in grid.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            match c {
                'E' => end = Pos::new(i as isize, j as isize),
                'S' => start = Pos::new(i as isize, j as isize),
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
    prev_direction: Option<Direction>,
    path: Vec<Pos>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_allowed_directions(current_dir: Option<Direction>) -> Vec<Direction> {
    if current_dir.is_none() {
        return vec![Direction::East, Direction::South, Direction::West, Direction::North];
    }

    match current_dir.unwrap() {
        Direction::East => vec![Direction::East, Direction::North, Direction::South],
        Direction::South => vec![Direction::South, Direction::East, Direction::West],
        Direction::West => vec![Direction::West, Direction::North, Direction::South],
        Direction::North => vec![Direction::North, Direction::East, Direction::West],
        _ => unimplemented!()
    }
}

fn dijkstra(start: Pos, goal: Pos, grid: &Vec<Vec<char>>) -> (Option<State>, HashSet<Pos>) {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut best_paths_cells: HashSet<Pos> = HashSet::new();
    let mut best_state: Option<State> = None;

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start, prev_direction: Some(Direction::East), path: vec![start] });

    while let Some(state) = heap.pop() {
        if state.position == goal {
            best_paths_cells.extend(&state.path);
            println!("New best path found for state {:?}", state);
            best_state = Some(state);
            continue;
        }

        let State { cost, position, prev_direction, path } = state;

        if dist.get(&position).is_some() && cost > *dist.get(&position).unwrap() {
            continue;
        }

        for direction in get_allowed_directions(prev_direction) {
            let new_pos = position.neighbor(direction);
            let (nx, ny) = (new_pos.x as usize, new_pos.y as usize);

            let c = grid[nx][ny];
            // Wall, skip this path
            if c == '#' {
                continue;
            }

            let mut next_cost = cost;
            if prev_direction.is_none() || direction == prev_direction.unwrap() {
                // If current direction is the same as previous, basic point
                next_cost += 1;
            } else {
                // Otherwise, it can only be a turn, it's over a thousand!
                next_cost += 1001;
            }

            if dist.get(&new_pos).is_none() || next_cost < *dist.get(&new_pos).unwrap() {
                *dist.entry(new_pos).or_default() += next_cost;
                let mut new_path = path.clone();
                new_path.push(new_pos);
                heap.push(State { cost: next_cost, position: new_pos, prev_direction: Some(direction), path: new_path });
            }
        }
    }

    (best_state, best_paths_cells)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start, end) = parse(input);

    let (state, _) = dijkstra(start, end, &grid);

    Some(state.unwrap().cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start, end) = parse(input);

    let (_, cells) = dijkstra(start, end, &grid);

    Some(cells.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
