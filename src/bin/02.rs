advent_of_code::solution!(2);

fn is_safe(level: &Vec<u32>) -> bool {
  // could be optimized using only one level.window to check both
  // take first window to check direction, then aggregate conditions
  let is_only_increasing = level.windows(2).all(|w| {
    let diff = w[0].abs_diff(w[1]);
    return w[0] < w[1] && diff >= 1 && diff <= 3
  });

  let is_only_decreasing = level.windows(2).all(|w| {
    let diff = w[0].abs_diff(w[1]);
    return w[0] > w[1] && diff >= 1 && diff <= 3
  });

  is_only_increasing || is_only_decreasing
}

pub fn part_one(_input: &str) -> Option<u32> {
    Some(
        _input
            .lines()
            .map(|l| {
                let split = l
                  .split_whitespace()
                  .map(|x| x.parse::<u32>().unwrap())
                  .collect::<Vec<u32>>();
                return is_safe(&split);
            })
            .filter(|x| *x)
            .collect::<Vec<_>>()
            .len() as u32,
    )
}

fn is_safe_part_two(vec: &Vec<u32>) -> bool {
  if is_safe(vec) {
    return true
  }

  // create new version without the number at index i to check if valid
  // Recursive approach ? Or clone is_safe to get which index is not safe
  // This would limit to three the possible new possibilities:
  // error pair first, last element, next pair first element
  for i in 0..vec.len() {
    let mut clone = vec.clone();
    clone.remove(i);

    if is_safe(&clone) {
      return true
    }
  }

  false
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(
        _input
          .lines()
          .map(|l| {
              let split = l
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            return is_safe_part_two(&split);
          })
          .filter(|x| *x)
          .collect::<Vec<_>>()
          .len() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4u32));
    }
}
