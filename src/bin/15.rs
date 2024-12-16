use crate::utils::{get_value_from_grid, parse_grid, Direction, Pos};
#[path = "utils/grid_utils.rs"] mod utils;

advent_of_code::solution!(15);

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Direction>, Pos) {
    let mut parts = input.split("\n\n");
    let matrix = parse_grid(parts.next().unwrap());
    let mut start_pos: Pos = Pos { x: 0, y: 0 };

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '@' {
                start_pos = Pos::new(i as isize, j as isize);
            }
        }
    }

    let moves: Vec<Direction> = parts.next().unwrap().chars().filter_map(|c| {
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

fn get_and_move_all(matrix: &Vec<Vec<char>>, pos: Pos, dir: Direction) -> Option<Vec<Pos>> {
    let mut zero_moved: Vec<Pos> = Vec::new();
    let mut next_pos = pos;
    let val = get_value_from_grid(&matrix, pos.x as usize, pos.y as usize).unwrap();

    loop {
        let new_pos = next_pos.neighbor(dir);
        let next_val = get_value_from_grid(&matrix, new_pos.x as usize, new_pos.y as usize);

        if next_val.is_none() || *next_val.unwrap() == '#' {
            return None;
        }

        zero_moved.push(new_pos);
        next_pos = new_pos;

        if *next_val.unwrap() == '.' {
            break;
        }
    }

    Some(zero_moved)
}

fn move_dir(matrix: &mut Vec<Vec<char>>, pos: Pos, dir: Direction) -> Pos {
    let new_pos = pos.neighbor(dir);

    if let Some(val) = get_value_from_grid(&matrix, new_pos.x as usize, new_pos.y as usize) {
        if *val == '[' {
            let box_second_part = new_pos.neighbor(Direction::East);
            if let Some(first_part) = get_and_move_all_zeroes(&matrix, new_pos, dir) {
                let second_part = get_and_move_all(&matrix, box_second_part, dir).unwrap();

                new_zeroes.iter().for_each(|&zero_pos| {
                    matrix[zero_pos.x as usize][zero_pos.y as usize] = 'O';
                });
                matrix[new_pos.x as usize][new_pos.y as usize] = '@';
                matrix[pos.x as usize][pos.y as usize] = '.';
                return new_pos;
            }
        } else if *val == ']' {
            let box_second_part = new_pos.neighbor(Direction::West);
            if let Some(first_part) = get_and_move_all_zeroes(&matrix, new_pos, dir) {
                let second_part = get_and_move_all_zeroes(&matrix, box_second_part, dir).unwrap();
                new_zeroes.iter().for_each(|&zero_pos| {
                    matrix[zero_pos.x as usize][zero_pos.y as usize] = 'O';
                });
                matrix[new_pos.x as usize][new_pos.y as usize] = '@';
                matrix[pos.x as usize][pos.y as usize] = '.';
                return new_pos;
            }
        } else if *val == 'O' {
            if let Some(new_zeroes) = get_and_move_all_zeroes(&matrix, new_pos, dir) {
                new_zeroes.iter().for_each(|&zero_pos| {
                    matrix[zero_pos.x as usize][zero_pos.y as usize] = 'O';
                });
                matrix[new_pos.x as usize][new_pos.y as usize] = '@';
                matrix[pos.x as usize][pos.y as usize] = '.';
                return new_pos;
            }
        } else if *val == '.' {
            matrix[new_pos.x as usize][new_pos.y as usize] = '@';
            matrix[pos.x as usize][pos.y as usize] = '.';
            return new_pos;
        }
    }

    pos
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut matrix, moves, pos) = parse(input);

    let mut current_pos = pos;

    moves.iter().for_each(|&dir| {
        current_pos = move_dir(&mut matrix, current_pos, dir);
    });

    let mut result = 0;
    matrix.iter().enumerate().for_each(|(i, l)| {
        l.iter().enumerate().for_each(|(j, &c)| {
            if c == 'O' {
                result += i * 100 + j;
            }
        })
    });

    for l in matrix {
        for c in l {
            print!("{}", c);
        }
        println!();
    }

    Some(result)
}

fn reconfigure_map(matrix: &Vec<Vec<char>>) -> str {
    matrix.iter().enumerate().map(
        |(i, l)| {
            l.iter().enumerate().map(
                |(j, c)| match c {
                    '@' => "@.",
                    '#' => "##",
                    'O' => "[]",
                    _ => "..",
                }
            ).join()
        }
    ).join('\n')
}

pub fn part_two(input: &str) -> Option<usize> {
    let (temp_matrix) = parse(input);
    // transform into new map, take new params into account ([] mostly)
    let new_str = reconfigure_map(temp_matrix);
    let (mut matrix, moves, pos) = parse(input);


    let mut current_pos = pos;

    moves.iter().for_each(|&dir| {
        current_pos = move_dir(&mut matrix, current_pos, dir);
    });

    let mut result = 0;
    matrix.iter().enumerate().for_each(|(i, l)| {
        l.iter().enumerate().for_each(|(j, &c)| {
            if c == '[' {
                result += i * 100 + j;
            }
        })
    });

    for l in matrix {
        for c in l {
            print!("{}", c);
        }
        println!();
    }

    Some(result)
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
