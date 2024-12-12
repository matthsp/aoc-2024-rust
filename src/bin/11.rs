use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn apply_rule(stone: i64) -> (i64, Option<i64>) {
    match stone {
        0 => (1, None),
        // Use power of ten to split number in two
        i if i.checked_ilog10().unwrap_or(0) % 2 == 1 => {
            let divisor = 10_i64.pow(i.checked_ilog10().unwrap_or(0) / 2 + 1);
            (i / divisor, Some(i % divisor))
        },
        _ => (stone * 2024, None)
    }
}

fn iterate_stones(stones: Vec<i64>, iteration_count: i8) -> i64 {
    let mut stone_map = stones.iter().map(|s| (*s, 1_i64)).collect();

    for _ in 0..iteration_count {
        let mut new_stones_map: HashMap<i64, i64> = HashMap::new();

        // For each stone, apply rule and add to map the new stone(s) and counts
        for (stone, count) in stone_map {
            let result = apply_rule(stone);
            *new_stones_map.entry(result.0).or_default() += count;
            if result.1.is_some() {
                *new_stones_map.entry(result.1.unwrap()).or_default() += count;
            }
        }

        stone_map = new_stones_map;
    }

    stone_map.values().sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let stones = parse(input);

    let result = iterate_stones(stones, 25_i8);

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let stones = parse(input);

    let result = iterate_stones(stones, 75_i8);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

// Awesome stuff learned today:
// Map stuff:
// * Vec to HashMap using (<key>, <val>) mapping: <vec>.iter().map(|s| (<key>,<val>)).collect();
// * Map or_default() automatically fill value with type
// * Increment value in map: *<var>.entry(<key>).or_default() += count;
//
// Numeric stuff:
// use power of tens to identify digit count in number:  <number>.checked_<type>log10().unwrap_or(0)
// use power of tens to split a number in two:
//     let divisor = 10_i64.pow(<number>.checked_<type>log10().unwrap_or(0) / 2 + 1);
//     let (first_part, second_part) = (<number>/divisor,<number>%divisor)
