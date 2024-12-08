use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

fn  parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<Pos>>) {
    let mut antenna_map: HashMap<char, Vec<Pos>> = HashMap::new();

    let matrix = input.lines().enumerate().map(|(i, l)| {
        let chars: Vec<char> = l.chars().collect();
        for (index, &c) in chars.iter().enumerate() {
            if c != '.' && c != '#' {
                antenna_map.entry(c)
                    .or_insert_with(Vec::new)
                    .push(Pos { x: i as i32, y: index as i32 });
            }
        }
        chars
    }).collect();

    (matrix, antenna_map)
}

fn is_inbound(matrix: &Vec<Vec<char>>, pos: Pos) -> bool {
    if pos.x >= 0 && pos.x < matrix.len() as i32 && pos.y >= 0 && pos.y < matrix[0].len() as i32 {
        return true;
    }
    false
}

fn process_antinodes(matrix: &Vec<Vec<char>>, vec: &Vec<Pos>, part_two: bool) -> HashSet<Pos> {
    // Ensure we dont count several time the same antinode
    let mut set: HashSet<Pos> = HashSet::new();

    for (i, item1) in vec.iter().enumerate() {
        if part_two {
            // ensure the first antenna itself is also added for part two
            set.insert(Pos { x: item1.x, y: item1.y });
        }
        for item2 in vec[i+1..].iter() {
            let height = item2.x - item1.x;
            let width = item2.y - item1.y;
            let mut factor = 1;

            if part_two {
                // ensure the second antenna itself is also added for part two
                set.insert(Pos { x: item2.x, y: item2.y });
            }

            loop {
                let mut result: u32 = 0;

                let f_antenna_sim = Pos { x: item2.x + height * factor, y: item2.y + width * factor };
                if is_inbound(matrix, f_antenna_sim) {
                    set.insert(f_antenna_sim);
                    result += 1;
                }
                let s_antenna_sim = Pos { x: item1.x - height * factor, y: item1.y - width * factor };
                if is_inbound(matrix, s_antenna_sim) {
                    set.insert(s_antenna_sim);
                    result += 1;
                }

                // if part one or no antinode is added, break the loop
                if !part_two || result == 0 {
                    break;
                }
                // Increase the factor to add more antinodes
                factor += 1;
            }
        }
    }
    set
}

pub fn part_one(input: &str) -> Option<usize> {
    let (matrix, map) = parse_input(input);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    map.into_values().for_each(
        |vec: Vec<Pos>| antinodes.extend(process_antinodes(&matrix, &vec, false))
    );


    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (matrix, map) = parse_input(input);
    let mut antinodes: HashSet<Pos> = HashSet::new();

    map.into_values().for_each(
        |vec: Vec<Pos>| antinodes.extend(process_antinodes(&matrix, &vec, true))
    );

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
