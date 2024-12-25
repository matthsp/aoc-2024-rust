use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::{Ordering, Reverse};
use itertools::Position;

#[path = "utils/grid_utils.rs"] mod utils;
use crate::utils::{parse_grid, Pos, Direction, CARDINAL_DIRECTIONS, is_in_boundaries, get_value_from_grid_pos, Maze};

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    position: Pos,
    direction: Direction,
    cost: usize,
    history: Option<Vec<Pos>>,
}

fn dijkstra_with_backtrack(
    grid: &Vec<Vec<char>>,
    start: Pos,
    end: Pos,
    min_cost: usize,
) -> usize {
    let mut to_visit = vec![vec![[min_cost, min_cost]; grid[0].len()]; grid.len()];
    let mut prio = BinaryHeap::new();
    let mut tiles = HashSet::new();
    to_visit[start.x as usize][start.y as usize][Direction::East.get_axis()] = 0;
    prio.push(Reverse(Tile {
        position: start,
        direction: Direction::East,
        cost: 0,
        history: Some(vec![]),
    }));
    while let Some(Reverse(Tile {
                               position,
                               direction,
                               cost,
                               history,
                           })) = prio.pop()
    {
        let mut history = history.unwrap();
        history.push(position);

        if cost > to_visit[position.x as usize][position.y as usize][direction.get_axis()] || cost > min_cost {
            continue;
        }

        if position == end {
            if cost == min_cost {
                tiles.extend(history);
            }

            continue;
        }
        for dir in CARDINAL_DIRECTIONS {
            let next_dir = dir;
            let next_pos = position.neighbor(dir);
            let mut next_cost = cost;
            if next_dir == direction {
                next_cost += 1;
            } else {
                next_cost += 1001;
            }
            if grid[next_pos.x as usize][next_pos.y as usize] == '#' {
                continue;
            }

            if next_cost <= to_visit[next_pos.x as usize][next_pos.y as usize][direction.get_axis()] {
                to_visit[next_pos.x as usize][next_pos.y as usize][direction.get_axis()] = next_cost;
                prio.push(Reverse(Tile {
                    position: next_pos,
                    direction: next_dir,
                    cost: next_cost,
                    history: Some(history.clone()),
                }));
            }
        }
    }

    tiles.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::from(input);

    let min_cost = maze.dijkstra();
    let tile_count = maze.dijkstra_with_backtrack(min_cost, &Direction::East);

    Some(tile_count)
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
