use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use crate::utils::{Direction, Pos, CARDINAL_DIRECTIONS};

#[path = "utils/grid_utils.rs"] mod utils;

advent_of_code::solution!(21);

fn get_dir_char(dir: &Direction) -> char {
    match dir {
        Direction::East => '^',
        Direction::West=> 'v',
        Direction::North => '<',
        Direction::South => '>',
        _ => unimplemented!()
    }
}

fn get_map_ref(grid: &Vec<Vec<char>>) -> HashMap<char, Pos> {
    grid.iter().enumerate().flat_map(
        |(i, l)| {
            l.iter().enumerate().filter_map(move |(j, &c)| {
                Some((c, Pos::new(i as isize, j as isize)))
            })
        }
    ).collect()
}

fn get_keypad() -> (Vec<Vec<char>>, Pos, HashMap<char, Pos>) {
    let keypad = vec![
        vec!['7','8','9',],
        vec!['4','5','6',],
        vec!['1','2','3',],
        vec![' ','0','A',],
    ];
    let map = get_map_ref(&keypad);

    (keypad, Pos::new(3, 2), map)
}

fn get_control_pad() ->  (Vec<Vec<char>>, Pos, HashMap<char, Pos>) {
    let control_pad = vec![
        vec![' ','^','A',],
        vec!['<','v','>',],
    ];
    let map = get_map_ref(&control_pad);
    (control_pad, Pos::new(0, 2), map)
}

fn find_shortest_paths(grid: &Vec<Vec<char>>, start: Pos, end: Pos) -> Vec<Vec<char>> {
    if start == end {
        return vec![vec!['A']];
    }

    let mut dists = vec![[usize::MAX; 3]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        dists[pos.x as usize][pos.y as usize] = steps;

        for dir in CARDINAL_DIRECTIONS {
            let neighbor = pos.neighbor_in_grid(dir, grid);
            if neighbor.is_none() {
                continue;
            }

            let (new_pos, val) = neighbor.unwrap();
            let nx = new_pos.x as usize;
            let ny = new_pos.y as usize;
            if val != ' ' && dists[nx][ny] == usize::MAX {
                queue.push_back((new_pos, steps + 1));
            }
        }
    }

    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((end, vec!['A']));

    while let Some((pos, path)) = stack.pop() {
        if pos == start {
            paths.push(path);
            continue;
        }
        for (i, dir) in CARDINAL_DIRECTIONS.iter().enumerate() {
            let neighbor = pos.neighbor_in_grid(*dir, grid);
            if neighbor.is_none() {
                continue;
            }

            let (new_pos, val) = neighbor.unwrap();
            let nx = new_pos.x as usize;
            let ny = new_pos.y as usize;

            if dists[nx][ny] < dists[pos.x as usize][pos.y as usize] {
                let dir_char = get_dir_char(dir);
                let mut new_path = vec![dir_char];
                new_path.extend(&path);
                stack.push((new_pos, new_path));
            }
        }
    }

    paths
}

fn find_shortest_sequence(
    input: &Vec<char>,
    depth: usize,
    highest: bool,
    cursors: &mut Vec<char>,
    keypad: &(Vec<Vec<char>>, Pos, HashMap<char, Pos>),
    control_pad: &(Vec<Vec<char>>, Pos, HashMap<char, Pos>),
    cache: &mut HashMap<(Vec<char>, usize, char), usize>,
) -> usize {
    let cache_key = (input.clone(), depth, cursors[depth]);

    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    for c in input {
        let (grid, _, map) = if highest { keypad } else { control_pad };
        let start = map.get(&cursors[depth]).unwrap();
        let end = *map.get(c).unwrap();

        let paths = find_shortest_paths(grid, *start, end);

        if depth == 0 {
            result += paths.into_iter().map(|l| l.len()).min().unwrap();
        } else {
            result += paths
                .into_iter()
                .map(|p| {
                    find_shortest_sequence(&p, depth - 1, false, cursors, keypad, control_pad, cache)
                })
                .min()
                .unwrap();
        }
        cursors[depth] = *c;
    }

    cache.insert(cache_key, result);

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let keypad = get_keypad();
    let control_pad = get_control_pad();
    let mut cache = HashMap::new();

    let mut total = 0;
    for l in input.lines() {

        let mut cursors = vec!['A'; 3];
        let len = find_shortest_sequence(
            &l.chars().collect(),
            2,
            true,
            &mut cursors,
            &keypad,
            &control_pad,
            &mut cache,
        );

        let n = l[0..3].parse::<usize>().unwrap();
        total += n * len;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
