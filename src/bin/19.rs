use std::collections::{HashSet};

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut sections = input.split("\n\n");

    let towel_patterns: HashSet<String> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs: Vec<String> = sections
        .next()
        .unwrap()
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    (towel_patterns, designs)
}

fn count_ways(patterns: &HashSet<String>, design: &str) -> usize {
    let design_length = design.len();
    let mut dp = vec![0; design_length + 1];
    // Default case (empty)
    dp[0] = 1;

    for i in 1..=design_length {
        for pattern in patterns {
            let pattern_len = pattern.len();
            if i >= pattern_len && &design[i - pattern_len..i] == pattern {
                dp[i] += dp[i - pattern_len];
            }
        }
    }

    dp[design_length]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns, designs) = parse_input(input);

    Some(designs
        .iter()
        .filter(|design| count_ways(&patterns, design) > 0)
        .count())
}


pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, designs) = parse_input(input);

    Some(designs
        .iter()
        .map(|design| count_ways(&patterns, design))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
