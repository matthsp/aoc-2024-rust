advent_of_code::solution!(4);

fn find_all_char_positions(s: &str, target: char) -> Vec<usize> {
    let result: Vec<usize> = s.char_indices()
        .filter_map(|(i, c)| if c == target { Some(i) } else { None })
        .collect();
    result
}

fn count_xmas(matrix: &Vec<Vec<char>>, position: &Position) -> i32 {
    let mut result = 0;
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x == 0 && y == 0 {
                continue;
            }
            result += count_xmas_in_direction(matrix, position, Position { x, y }) as i32;
        }
    }
    result
}

fn count_xmas_in_direction(matrix: &Vec<Vec<char>>, position: &Position, direction: Position) -> bool {
    for (factor, letter) in [(1,'M'), (2,'A'), (3,'S')] {
        let new_x = position.x + (direction.x * factor);
        let new_y = position.y + (direction.y * factor);

        // Ensure the new positions are within the matrix bounds
        if new_x < 0 || new_x >= matrix.len() as isize || new_y < 0 || new_y >= matrix[0].len() as isize {
            return false;
        }

        if matrix[new_x as usize][new_y as usize] != letter {
            return false;
        }
    }
    true
}

struct Position {
    x: isize,
    y: isize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut x_positions: Vec<Position> = Vec::new();

    let matrix: Vec<Vec<char>> = input.lines()
        .enumerate()
        .map(|(index, line)| {
            x_positions.extend(find_all_char_positions(line, 'X').iter().map(|p| Position { x: index as isize, y: *p as isize }));
            line.chars().collect()
        }).collect();

    Some(x_positions.iter().map(
        |p: &Position| count_xmas(&matrix, p) as u32
    ).sum())
}

fn count_x_mas(matrix: &Vec<Vec<char>>, position: &Position) -> i32 {
    let mut result = 0;

    for dir in [-1, 1] {
        result += count_x_mas_in_direction(matrix, position, Position { x: dir, y: 0 }) as i32;
        result += count_x_mas_in_direction(matrix, position, Position { x: 0, y: dir }) as i32;
    }

    result
}


fn count_x_mas_in_direction(matrix: &Vec<Vec<char>>, position: &Position, direction: Position) -> bool {
    let rl_mode = direction.x == 0;

    let mut m_pos: Vec<Position> = Vec::new();
    let mut s_pos: Vec<Position> = Vec::new();
    if rl_mode {
        let new_y =
        m_pos.extend([ Position { x: position.x + 1, y: position.y + direction.y }, Position { x: position.x - 1, y: position.y + direction.y } ]);
        s_pos.extend([ Position { x: position.x + 1, y: position.y - direction.y }, Position { x: position.x - 1, y: position.y - direction.y } ]);
    } else {
        m_pos.extend([ Position { x: position.x  + direction.x, y: position.y + 1 }, Position { x: position.x + direction.x, y: position.y - 1 } ]);
        s_pos.extend([ Position { x: position.x  - direction.x, y: position.y + 1 }, Position { x: position.x - direction.x, y: position.y - 1 } ]);
    }

    for pos in m_pos {
        // Ensure the new positions are within the matrix bounds
        if pos.x < 0 || pos.x >= matrix.len() as isize || pos.y < 0 || pos.y >= matrix[0].len() as isize {
            return false;
        }

        if matrix[pos.x as usize][pos.y as usize] != 'M' {
            return false;
        }
    }

    for pos in s_pos {
        // Ensure the new positions are within the matrix bounds
        if pos.x < 0 || pos.x >= matrix.len() as isize || pos.y < 0 || pos.y >= matrix[0].len() as isize {
            return false;
        }

        if matrix[pos.x as usize][pos.y as usize] != 'S' {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut a_positions: Vec<Position> = Vec::new();

    let matrix: Vec<Vec<char>> = input.lines()
        .enumerate()
        .map(|(index, line)| {
            a_positions.extend(find_all_char_positions(line, 'A').iter().map(|p| Position { x: index as isize, y: *p as isize }));
            line.chars().collect()
        }).collect();

    Some(a_positions.iter().map(
        |p: &Position| count_x_mas(&matrix, p) as u32
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
