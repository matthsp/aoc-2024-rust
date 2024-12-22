use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn calculate_price(mut x: u64) -> u64 {
    for _ in 0..2000 {
        x = (x.wrapping_mul(64) ^ x) % 16777216;
        x = (x.wrapping_div(32) ^ x) % 16777216;
        x = (x.wrapping_mul(2048) ^ x) % 16777216;
    }
    x
}

fn calculate_real_price(mut x: u64, cache: &mut HashMap<Vec<i8>, u64>) -> u64 {
    let mut price = 0;
    let mut sequence: Vec<(u8,i8)> = Vec::new();
    let mut visited: HashSet<Vec<i8>> = HashSet::new();
    for i in 0..2000 {
        x = (x.wrapping_mul(64) ^ x) % 16777216;
        x = (x.wrapping_div(32) ^ x) % 16777216;
        x = (x.wrapping_mul(2048) ^ x) % 16777216;

        let new_price = x % 10;
        let diff = (new_price as i64 - price as i64) as i8;
        price = new_price;

        if i > 0 {
            sequence.push((new_price as u8, diff));
        }

        // register the sequence, then get the last four diff
        if i > 3 {
            let cache_key = vec![sequence[i - 4].1, sequence[i - 3].1, sequence[i - 2].1, diff];
            if !visited.contains(&cache_key) {
                visited.insert(cache_key.clone());
                *cache.entry(cache_key).or_default() += new_price;
            }
        }
    }
    x
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| {
        let secret = l.parse::<_>().unwrap();
        calculate_price(secret)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache: HashMap<Vec<i8>, u64> = HashMap::new();

    input.lines().for_each(|l| {
        let secret = l.parse::<_>().unwrap();
        let _ = calculate_real_price(secret, &mut cache);
    });

    let max_bananas = cache.iter().max_by_key(|(key, &val)| val);
    Some(*max_bananas.unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
