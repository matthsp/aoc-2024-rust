use std::collections::HashMap;

advent_of_code::solution!(1);


struct Occurrence {
    left: u32,
    right: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = input.lines().map(
        |l| {
            let mut split = l.split_whitespace();
            let left = split.next();
            let right = split.next();

            (
                left.unwrap().parse::<u32>().unwrap(),
                right.unwrap().parse::<u32>().unwrap(),
            )
        }
    ).collect::<(Vec<_>, Vec<_>)>();

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right)
            .map(|(a,b)| a.abs_diff(b))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut occurrences = HashMap::new();

    for part in input.lines() {
        let mut split = part.split_whitespace().map(|a| a.parse::<u32>().unwrap());
        let l = split.next().unwrap();
        let r = split.next().unwrap();

        occurrences
            .entry(l)
            .and_modify(|e: &mut Occurrence| e.left += 1)
            .or_insert(Occurrence { left: 1, right: 0 });

        occurrences.entry(r)
            .and_modify(|e: &mut Occurrence| e.right += 1)
            .or_insert(Occurrence { left: 0, right: 1 });
    }

    Some(
        occurrences.iter()
            .enumerate()
            .map(|(_, (key, occurrence))| {
                key * occurrence.left * occurrence.right
            })
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31u32));
    }
}
