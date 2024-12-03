use regex::Regex;

advent_of_code::solution!(3);

fn apply_mul_functions(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input).map(|_match| {
        &_match[1].parse::<u32>().unwrap() * &_match[2].parse::<u32>().unwrap()
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(apply_mul_functions(input))
}

fn apply_do_dont(input: &str) -> u32 {
    let re = Regex::new(r"(?P<mul>mul\((\d{1,3}),(\d{1,3})\))|(?P<doing>do\(\))|(?P<dont>don\'t\(\))").unwrap();

    let mut activated = true;
    re.captures_iter(input).filter_map(|_match| {
        if let Some(_) = _match.name("mul") {
            if activated {
                return Some(&_match[2].parse::<u32>().unwrap() * &_match[3].parse::<u32>().unwrap())
            }
        } else if _match.name("doing").is_some() {
            activated = true;
        } else if _match.name("dont").is_some() {
            activated = false;
        };
        None
    }).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(apply_do_dont(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
