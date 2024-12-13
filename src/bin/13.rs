use itertools::Itertools;

advent_of_code::solution!(13);

struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

fn parse(input: &str) -> Vec<Machine> {
    // Split per machine, then fetch tuple of coordinates
    input.split("\n\n").map(|l| {
        let (x1, x2, y1, y2, z1, z2) = l
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Machine {
            button_a: (x1, y1),
            button_b: (x2, y2),
            prize: (z1, z2),
        }
    }).collect()
}

// https://en.wikipedia.org/wiki/Cramer%27s_rule
fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        // No solution
        return 0;
    }
    // Calculate price (3 * button a + button b)
    a * 3 + b
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse(input);

    let mut result = 0;
    for machine in machines.iter() {
        // Extract coefficients
        let (x1, y1) = machine.button_a;
        let (x2, y2) = machine.button_b;
        let (z1, z2) = machine.prize;

        result += solve(x1 as i64, x2 as i64, y1 as i64, y2 as i64, z1 as i64, z2 as i64)

    }

    Some(result) // Return a placeholder value for now
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse(input);

    let mut result = 0;
    for machine in machines.iter() {
        // Extract coefficients
        let (x1, y1) = machine.button_a;
        let (x2, y2) = machine.button_b;
        let (z1, z2) = machine.prize;
        let (fixed_z1, fixed_z2) = (
            z1 as i64 + 10000000000000i64,
            z2 as i64 + 10000000000000i64
        );
        result += solve(x1 as i64, x2 as i64, y1 as i64, y2 as i64, fixed_z1, fixed_z2)

    }

    Some(result) // Return a placeholder value for now
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
