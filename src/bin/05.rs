use std::cmp::Ordering::{Greater, Less};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn split_file(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let splitted: Vec<&str> = input.split("\n\n").collect();

    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for ordering in splitted[0].lines() {
        let pair: Vec<u32> = ordering.split('|')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        page_ordering_rules.entry(pair[1]).or_insert_with(Vec::new).push(pair[0]);
    }

    let update_page_numbers: Vec<Vec<u32>> = splitted[1].lines().map(
        |l| l.split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect()
    ).collect();

    (page_ordering_rules.into_iter().collect(), update_page_numbers)
}

fn is_update_valid(input: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut checked: HashSet<u32> = HashSet::new();
    let mut to_check: Vec<u32> = input.clone();

    while let Some(page) = to_check.pop() {
        if let Some(values) = rules.get(&page) {
            if values.iter().any(|&value| checked.contains(&value)) {
                return false;
            }
        }
        checked.insert(page);
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (page_ordering_rules, update_page_numbers) = split_file(input);

    let mut result: u32 = 0;
    for update in update_page_numbers {
        if is_update_valid(&update, &page_ordering_rules) {
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

fn reorder_update(input: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut to_sort = input.clone();
    to_sort.sort_by(|a, b| {
        match rules.get(a) {
            Some(value) if value.contains(b) => Greater,
            _ => Less,
        }
    });
    to_sort
}

pub fn part_two(input: &str) -> Option<u32> {
    let (page_ordering_rules, update_page_numbers) = split_file(input);

    let mut result: u32 = 0;
    for update in update_page_numbers {
        if !is_update_valid(&update, &page_ordering_rules) {
            let ordered_update: Vec<u32> = reorder_update(&update, &page_ordering_rules);
            result += ordered_update[ordered_update.len() / 2];
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
