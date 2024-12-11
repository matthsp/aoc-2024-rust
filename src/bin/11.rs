use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<String> {
    input.trim().split_whitespace().map(String::from).collect()
}

fn apply_rule(stone: &str) -> Vec<String> {
    match stone {
        "0" => vec!["1".to_string()],
        s if s.len() % 2 == 0 => {
            let mid = s.len() / 2;
            vec![s[..mid].to_string(), s[mid..].parse::<usize>().unwrap().to_string()]
        },
        _ => vec![(stone.parse::<usize>().unwrap() * 2024).to_string()]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = parse(input);

    for _ in 0..25 {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in stones.iter() {
            new_stones.extend(apply_rule(stone));
        }
        stones = new_stones;
    }

    Some(stones.len() as u32)
}

fn apply_rule_number_based(stone: i64) -> (i64, Option<i64>) {
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



pub fn part_two(input: &str) -> Option<i64> {
    let stones: Vec<i64> = input.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    // Save the count for each stone (index: stone, value: count for current iteration)
    let mut stones_map: HashMap<i64, i64> = stones.iter().map(|s| (*s, 1_i64)).collect();

    for _ in 0..75 {
        let mut new_stones_map: HashMap<i64, i64> = HashMap::new();

        // For each stone, apply rule and add to map the new stone(s) and counts
        for (stone, count) in stones_map {
            let result = apply_rule_number_based(stone);
            *new_stones_map.entry(result.0).or_default() += count;
            if result.1.is_some() {
                *new_stones_map.entry(result.1.unwrap()).or_default() += count;
            }
        }

        stones_map = new_stones_map;
    }

    // Fetch all stone counts
    Some(stones_map.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
