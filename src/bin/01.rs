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

    let mut diff: u32 = 0;
    for (i, l) in left.iter().enumerate() {
        diff += l.abs_diff(right[i]);
    }

    Some(
        left.iter()
            .zip(right)
            .map(|(a,b)| a.abs_diff(b))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut occurences = HashMap::new();

    let parts = input.lines();
    for (i, part) in parts.enumerate() {
        let sides: Vec<&str> = part.split_whitespace().collect();
        if sides.len() == 2 {
            let [l, r] = [sides[0], sides[1]];
            if occurences.get(l).is_none() {
                occurences.insert(l, Occurrence { left: 1, right: 0 });
            } else {
                occurences.insert(l, Occurrence { left: occurences.get(l)?.left + 1, right: occurences.get(l)?.right });
            }

            if !occurences.get(r).is_none() {
                occurences.insert(r, Occurrence { left: occurences.get(r)?.left, right: occurences.get(r)?.right + 1 }  );
            } else {
                occurences.insert(r, Occurrence { left: 0, right: 1 }  );
            }
        }
    }

    let mut diff: u32 = 0;
    for (lr) in occurences.iter() {
        diff += lr.0.parse::<u32>().unwrap() * lr.1.left * lr.1.right;
    }

    Some(diff)
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
