advent_of_code::solution!(7);

pub fn parse(input: &str) -> Vec<(i64, &str)> {
    input.lines().map(|l| {
        let mut parts = l.split(':');
        let result = parts.next().unwrap().parse::<i64>().unwrap();

        let numbers = parts.next()
            .unwrap()
            .trim();

        (result, numbers)
    }).collect()
}

#[derive(Debug, PartialEq)]
enum Operation {
    MUL,
    ADD,
    PIP,
}

fn check_equation(expected_result: i64, acc: i64, numbers: &Vec<i64>, index: usize, operation: Operation, is_part_two: bool) -> bool {
    if index >= numbers.len() {
        return false
    }

    let next_number = numbers[index];
    let next_acc= match operation {
        Operation::ADD => acc + next_number,
        Operation::MUL => acc * next_number,
        Operation::PIP => format!("{}{}", acc, next_number).parse::<i64>().unwrap(),
    };

    if next_acc == expected_result && index + 1 == numbers.len() {
        return true;
    }
    if next_acc > expected_result || (next_acc < expected_result && index + 1 == numbers.len() ) {
        return false
    }

    let mut result =  check_equation(expected_result, next_acc, numbers, index + 1, Operation::ADD, is_part_two);
    if result {
        return true;
    }
    result = check_equation(expected_result, next_acc, numbers, index + 1, Operation::MUL, is_part_two);
    if result || !is_part_two {
        return result
    }

    check_equation(expected_result, next_acc, numbers, index + 1, Operation::PIP, is_part_two)
}

pub fn part_one(input: &str) -> Option<i64> {
    let potential_equations = parse(input);
    Some(potential_equations.iter().filter_map(
        |(expected_result, numbers)| {
            let split: Vec<i64> = numbers.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            let mut result = check_equation(*expected_result, split[0], &split, 1, Operation::ADD, false);
            if !result {
                result = check_equation(*expected_result, split[0], &split, 1, Operation::MUL, false);
            }
            if result {
                return Some(*expected_result);
            }
            None
        }
    ).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let potential_equations = parse(input);
    Some(potential_equations.iter().filter_map(
        |(expected_result, numbers)| {
            let split: Vec<i64> = numbers.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            let mut result = check_equation(*expected_result, split[0], &split, 1, Operation::ADD, true);
            if !result {
                result = check_equation(*expected_result, split[0], &split, 1, Operation::MUL, true);
            }
            if !result {
                result = check_equation(*expected_result, split[0], &split, 1, Operation::PIP, true);
            }
            if result {
                return Some(*expected_result);
            }
            None
        }
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
