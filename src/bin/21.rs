use std::collections::HashMap;
use std::hash::Hash;
use crate::utils::{Direction, Pos};

#[path = "utils/grid_utils.rs"] mod utils;

advent_of_code::solution!(21);


fn get_dir_char(is_x: bool, is_positive: bool) -> char{
    match (is_x, is_positive) {
        (true, false) => '^',
        (true, true) => 'v',
        (false, false) => '<',
        (false, true) => '>',
    }
}

struct Robot {
    current_pos: Pos,
    current_char: char,
    keypad: HashMap<char, Pos>,
}

impl Robot {

    pub fn new(keypad: &HashMap<char, Pos>) -> Self {
        Robot { keypad: keypad.clone(), current_pos: *keypad.get(&'A').unwrap(), current_char: 'A' }
    }

    pub fn move_to(&mut self, key: char) -> Vec<char> {
        let mut movements: Vec<char> = Vec::new();
        let key_pos =  *self.keypad.get(&key).unwrap();
        let movement = key_pos - self.current_pos;
        let (x, y) = (movement.x, movement.y);

        if self.keypad.len() == 12 {
            if x < 0 && y < 0 {
                movements.extend(std::iter::repeat(get_dir_char(true, x > 0)).take(x.abs() as usize));
                movements.extend(std::iter::repeat(get_dir_char(false, y > 0)).take(y.abs() as usize));
            } else {
                movements.extend(std::iter::repeat(get_dir_char(false, y > 0)).take(y.abs() as usize));
                movements.extend(std::iter::repeat(get_dir_char(true, x > 0)).take(x.abs() as usize));
            }
        } else {
            if x < 0 && y > 0  {
                movements.extend(std::iter::repeat(get_dir_char(false, y > 0)).take(y.abs() as usize));
                movements.extend(std::iter::repeat(get_dir_char(true, x > 0)).take(x.abs() as usize));
            } else {
                movements.extend(std::iter::repeat(get_dir_char(true, x > 0)).take(x.abs() as usize));
                movements.extend(std::iter::repeat(get_dir_char(false, y > 0)).take(y.abs() as usize));
            }
        }

        // store new pos in robot memory
        self.current_pos = key_pos;
        self.current_char = key;

        movements
    }

    pub fn apply_full_instruction(&mut self, chars: &Vec<char>) -> Vec<char> {
        println!("Keypad size: {}", self.keypad.len());
        let mut result = Vec::new();

        chars.iter().for_each(
            |c| {
                result.extend(self.move_to(*c));
                result.push('A');
            }
        );

        result
    }
}

fn get_keypad() -> HashMap<char, Pos> {
    vec![
        vec!['7','8','9',],
        vec!['4','5','6',],
        vec!['1','2','3',],
        vec![' ','0','A',],
    ].iter().enumerate().flat_map(
        |(i, l)| {
            l.iter().enumerate().filter_map(move |(j, &c)| {
                Some((c, Pos::new(i as isize, j as isize)))
            })
        }
    ).collect()
}

fn get_control_pad() -> HashMap<char, Pos> {
    vec![
        vec![' ','^','A',],
        vec!['<','v','>',],
    ].iter().enumerate().flat_map(
        |(i, l)| {
            l.iter().enumerate().filter_map(move |(j, &c)| {
                Some((c, Pos::new(i as isize, j as isize)))
            }) }
    ).collect()
}

fn enter_code(input: &str) -> Vec<char> {
    let keypad_robot = Robot::new(&get_keypad());
    let control_pad_robot1 = Robot::new(&get_control_pad());
    // Or am I? :eyes:
    let human_control_pad = Robot::new(&get_control_pad());

    let bot_chain = vec![keypad_robot, control_pad_robot1, human_control_pad];
    let mut char_input: Vec<char> = input.chars().collect();

    for mut bot in bot_chain {
        char_input = bot.apply_full_instruction(&char_input);
        let instructions_string: String = char_input.iter().collect();
        println!("{} - {}", input, instructions_string);
    }

    char_input
}
fn get_numeric_val(input: &str) -> usize {
    let numeric_str: String = input.chars().filter(|c| c.is_digit(10)).collect();
    numeric_str.parse::<usize>().unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(
        |l| {
            let instructions = enter_code(l);
            println!("{} - {} * {} = {}", l , instructions.len() , get_numeric_val(l), instructions.len() * get_numeric_val(l));
            instructions.len() * get_numeric_val(l)
        }
    ).sum())
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
