use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    val: usize,
}

struct Trail {
    nodes: HashMap<Pos, Vec<Pos>>,
    trail_end_count: usize,
}

impl Trail {
    fn add_node(&mut self, pos: Pos) {
        self.nodes.entry(pos).or_insert(Vec::new());
    }

    fn has_node(&mut self, pos: &Pos) -> bool {
        self.nodes.get(pos).is_some()
    }

    fn add_edge(&mut self, pos1: Pos, pos2: Pos) {
        if !self.has_edge(pos1, pos2) {
            self.nodes.entry(pos1).or_insert_with(Vec::new);
            self.nodes.entry(pos2).or_insert_with(Vec::new);

            self.nodes.get_mut(&pos1).unwrap().push(pos2);
            self.nodes.get_mut(&pos2).unwrap().push(pos1);
        }
    }

    fn has_edge(&self, pos1: Pos, pos2: Pos) -> bool {
        if let Some(neighbors) = self.neighbors(&pos1) {
            neighbors.contains(&pos2)
        } else {
            false
        }
    }

    fn neighbors(&self, pos: &Pos) -> Option<&Vec<Pos>> {
        self.nodes.get(pos)
    }
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Pos>) {
    let mut trailheads: Vec<Pos> = Vec::new();

    let matrix = input.lines().enumerate()
        .map(|(x, l)| l.chars().enumerate().map(|(y, c)| {
            if c == '0' {
                trailheads.push(Pos { x, y, val: 0 });
            }
            c.to_digit(10).unwrap() as usize
        }).collect())
        .collect() ;

    (matrix, trailheads)
}

fn get_new_node(matrix: &Vec<Vec<usize>>, trailhead: Pos, offset: (isize, isize)) -> Option<Pos> {
    let x = trailhead.x as isize + offset.0;
    let y = trailhead.y as isize + offset.1;
    if x >= 0 && x < matrix.len() as isize && y >= 0 && y < matrix.len() as isize {
        if matrix[x as usize][y as usize] == trailhead.val + 1 {
            return Some(Pos { x: x as usize, y: y as usize, val: matrix[x as usize][y as usize] });
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let (matrix, trailheads) = parse(input);

    Some(trailheads.iter().map(
        |trailhead| {
            let mut trail = Trail { trail_end_count: 0, nodes: HashMap::new() };
            traverse_dfs(&matrix, &mut trail, trailhead.clone());
            trail.trail_end_count
        }
    ).sum())
}

fn traverse_dfs(matrix: &Vec<Vec<usize>>, trail: &mut Trail, trailhead: Pos) -> usize {
    if trailhead.val == 9 { return 1; }
    let mut path_count = 0;

    for offset in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
        if let Some(pos) = get_new_node(matrix, trailhead, *offset) {
            if pos.val == 9 && !trail.has_node(&pos) {
                trail.trail_end_count += 1;
            }
            trail.add_node(pos);
            trail.add_edge(trailhead, pos);
            path_count += traverse_dfs(matrix, trail, pos);
        }
    }
    path_count
}

pub fn part_two(input: &str) -> Option<usize> {
    let (matrix, trailheads) = parse(input);

    Some(trailheads.iter().map(
        |trailhead| {
            let mut trail = Trail { trail_end_count: 0, nodes: HashMap::new() };
            traverse_dfs(&matrix, &mut trail, trailhead.clone())
        }
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
