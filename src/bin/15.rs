#[path = "utils/grid_utils.rs"] mod utils;
use crate::utils::{Direction, Pos};

advent_of_code::solution!(15);

fn parse(input: &str, is_part_two: bool) -> (Vec<Vec<char>>, Vec<Direction>, Pos) {
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut matrix: Vec<Vec<char>>;
    let mut start_pos: Pos = Pos { x: 0, y: 0 };

    if !is_part_two {
        matrix = grid.lines().enumerate().map(
            |(x, l)| {
                l.chars().enumerate().map(|(y, c)| {
                    if c == '@' {
                        start_pos = Pos::new(x as isize, y as isize);
                        return '.';
                    }
                    c
                }).collect()
            }
        ).collect();
    } else {
        matrix = grid.lines().enumerate().map(
            |(x, l)| {
                l.chars().enumerate().flat_map(|(y, c)| {
                    match c {
                        '@' => {
                            start_pos = Pos::new(x as isize, y as isize);
                            ['.', '.']
                        },
                        'O' => ['[', ']'],
                        val => [val, val]
                    }
                }).collect()
            }
        ).collect();
    }

    let moves: Vec<Direction> = movements.chars().filter_map(|c| {
        match c {
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            'v' => Some(Direction::South),
            '^' => Some(Direction::North),
            _ => None,
        }
    }).collect();

    (matrix, moves, start_pos)
}

fn calculate_coordinates_sum(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter()
        .enumerate()
        .map(
            |(x, l)| {
               l.iter()
                   .enumerate()
                   .filter_map(
                       |(y, c)| {
                           if *c == 'O' {
                               return Some(x * 100 + y);
                           }
                           None
                       }
                   )
                   .sum::<usize>()
            }
        ).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut matrix, moves, start) = parse(input, false);

    let mut current_pos = start.clone();
    for direction in moves {
        let neighbor = current_pos.neighbor(direction);

        match matrix[neighbor.x as usize][neighbor.y as usize] {
            '.' => current_pos = neighbor,
            'O' => {
                let mut boxes = vec![neighbor];
                let mut next_cell = neighbor.neighbor(direction);

                // Search for the complete list of boxes in the same direction
                while matrix[next_cell.x as usize][next_cell.y as usize] == 'O' {
                    boxes.push(next_cell);
                    next_cell = next_cell.neighbor(direction);
                }

                // Move 'everything' if there is some available space
                if matrix[next_cell.x as usize][next_cell.y as usize] == '.' {
                    // In reality, we replace the neighbor pos by '.' (previous first box)
                    matrix[neighbor.x as usize][neighbor.y as usize] = '.';
                    // And push a new box in next_cell
                    matrix[next_cell.x as usize][next_cell.y as usize] = 'O';
                    current_pos = neighbor;
                }
            }
            // Do nothing otherwise, it is a wall
            _ => {}
        }
    }

    Some(calculate_coordinates_sum(&matrix))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (matrix, moves, start) = parse(input, false);

    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
