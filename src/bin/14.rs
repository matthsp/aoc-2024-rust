use std::collections::HashMap;
use std::ops::Mul;
use std::cmp::Ordering::*;
use crate::utils::Pos;

#[path = "utils/grid_utils.rs"] mod utils;

advent_of_code::solution!(14);

fn parse(input: &str) -> Vec<(Pos, Pos)> {
    input.lines().map(
        |l| {
            let parts: Vec<isize> = l.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            (Pos::new(parts[0], parts[1]), Pos::new(parts[2], parts[3]))
        }
    ).collect()
}

fn move_robot(robot: &(Pos, Pos), dimensions: (usize, usize), time: usize) -> Pos {
    let (initial_pos, vec) = robot;
    let (max_x, max_y) = dimensions;

    let new_x = (initial_pos.x + vec.x.mul(time as isize)).rem_euclid(max_x as isize);
    let new_y = (initial_pos.y + vec.y.mul(time as isize)).rem_euclid(max_y as isize);

    Pos::new(new_x, new_y)
}

fn get_quadrant_id(pos: Pos, width: usize, height: usize) -> Option<usize> {
    let height_quadrant = ((height- 1) / 2) as isize;
    let width_quadrant = ((width- 1) / 2) as isize;

    if pos.x == width_quadrant && pos.y == height_quadrant {
        return None;
    }

    if pos.x > width_quadrant {
        if pos.y > height_quadrant {
            return Some(3);
        } else if pos.y < height_quadrant {
            return Some(1);
        }
    } else if pos.x < width_quadrant {
        if pos.y > height_quadrant {
            return Some(2);
        } else if pos.y < height_quadrant {
            return Some(0)
        }
    }
    None
}

pub fn part_one_with_params(input: &str, width: usize, height: usize, time: usize) -> Option<usize> {
    let robots = parse(input);


    let mut result_pos_map: HashMap<usize, usize> = HashMap::new();
    robots.iter().for_each(
        |robot| {
            let new_pos = move_robot(robot, (width, height), time);

            let quadrant_id = get_quadrant_id(new_pos, width, height);

            if quadrant_id.is_some() {
                *result_pos_map.entry(quadrant_id.unwrap()).or_default() += 1;
            }
        }
    );

    result_pos_map.values().copied().reduce(|acc, val| acc * val)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_params(input, 101, 103, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse(input);
    let mut i = 0;

    loop {
        i += 1;

        // First pattern found (Vertical) at 2, then 103, 204, etc.
        // Horizontal pattern where starting at 75, then 178, etc.
        // I estimated around 6870-80 the tree should appear,
        // and got it right by analyzing the results around there
        if i % 101 == 2 {
            println!();
            println!("---------------{}---------------",i);
            println!();
        }
        let mut new_robots: Vec<(Pos, Pos)> = Vec::new();
        robots.iter().for_each(
            |robot| {
                new_robots.push((move_robot(robot, (101, 103), 1), robot.1))
            }
        );

        if i % 101 == 2 {
            display_grid(new_robots.iter().map(|r| r.0).collect(), 101, 103);
        }

        robots = new_robots.clone();

        if i > 6870 { break }
    }

    Some(0)
}

fn display_grid(positions: Vec<Pos>, width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    for pos in positions {
        if pos.x < width as isize && pos.y < height as isize {
            grid[pos.y as usize][pos.x as usize] = 'X';
        }
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_params(
            &advent_of_code::template::read_file("examples", DAY),
            11,
            7,
            100
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
