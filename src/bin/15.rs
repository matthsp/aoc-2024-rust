#[path = "utils/grid_utils.rs"] mod utils;
use crate::utils::{get_value_from_grid_pos, Direction, Pos};

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
                            start_pos = Pos::new(x as isize, y as isize * 2);
                            ['@', '.']
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
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
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
                           } else if *c == '[' {
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
                let mut next_cell = neighbor.neighbor(direction);

                // Traverse all boxes in the same direction
                while matrix[next_cell.x as usize][next_cell.y as usize] == 'O' {
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
            },
            // Do nothing otherwise, it is a wall
            _ => {},
        }
    }

    Some(calculate_coordinates_sum(&matrix))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut matrix, moves, start) = parse(input, true);

    let mut current_pos = start.clone();
    for direction in moves {
        let neighbor = current_pos.neighbor(direction);

        match get_value_from_grid_pos(&matrix, neighbor) {
            Some('.') => {
                matrix[current_pos.x as usize][current_pos.y as usize] = '.';
                matrix[neighbor.x as usize][neighbor.y as usize] = '@';
                current_pos = neighbor;
            },
            Some(val @ '[') | Some(val @ ']') => {
                let mut boxes = Vec::new();
                let mut blocked = false;
                boxes.push(neighbor);

                // Add both sides of the first box to a memory vec
                if *val == '[' {
                    boxes.push(neighbor.neighbor(Direction::East));
                } else {
                    boxes.push(neighbor.neighbor(Direction::West));
                }

                match direction {
                    // Similar process as for part one
                    Direction::West | Direction::East => {

                        // Traverse all boxes in the same direction. A box is two cell wide ('[', ']')
                        let mut next_cell = neighbor.neighbor(direction).neighbor(direction);

                        // Search for the complete list of boxes in the same direction
                        while matrix[next_cell.x as usize][next_cell.y as usize] == '[' ||
                            matrix[next_cell.x as usize][next_cell.y as usize] == ']' {
                            boxes.push(next_cell);
                            next_cell = next_cell.neighbor(direction);
                        }

                        // Move 'everything' if there is some available space
                        if matrix[next_cell.x as usize][next_cell.y as usize] != '.' {
                            blocked = true
                        }
                    },
                    // There can be up to two boxes behind a box, this case is harder
                    Direction::North | Direction::South => {
                        let mut current = boxes.clone();

                        while current.len() > 1 {
                            let mut next = Vec::new();

                            for b in current {
                                let path = b.neighbor(direction);

                                match matrix[path.x as usize][path.y as usize] {
                                    '#' => {
                                        // If we found a wall above/below the box we're searching, that
                                        // means we're blocked, and we can stop the search here.
                                        blocked = true;
                                        next.clear();
                                        break;
                                    }
                                    side @ '[' | side @ ']' => {
                                        // If we found another box, add it to our total list of boxes and
                                        // the list of next boxes to search if we don't already have it.
                                        if !next.contains(&path) {
                                            boxes.push(path);
                                            next.push(path);

                                            if side == '[' {
                                                boxes.push(path.neighbor(Direction::East));
                                                next.push(path.neighbor(Direction::East));
                                            } else {
                                                boxes.push(path.neighbor(Direction::West));
                                                next.push(path.neighbor(Direction::West));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            current = next;
                        }
                    },
                    // Should not happen but there are more directions available in Direction
                    _ => {}
                }


                if !blocked {
                    // Impossible to optimize like in part one, every box needs to be moved here
                    for b in boxes.iter().rev() {
                        let n = b.neighbor(direction);
                        matrix[n.x as usize][n.y as usize] = matrix[b.x as usize][b.y as usize];
                        matrix[b.x as usize][b.y as usize] = '.';
                    }

                    matrix[current_pos.x as usize][current_pos.y as usize] = '.';
                    matrix[neighbor.x as usize][neighbor.y as usize] = '@';

                    current_pos = neighbor
                }

            },
            // Do nothing otherwise, it is a wall
            _ => {}
        }
    }
    Some(calculate_coordinates_sum(&matrix))
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
        assert_eq!(result, Some(9021));
    }
}
