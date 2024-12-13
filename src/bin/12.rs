use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize
}

struct Polygon {
    coordinates: HashSet<Pos>,
    area: i32,
    perimeter: i32,
}

fn flood_fill( matrix: &Vec<Vec<char>>, start_i: usize, start_j: usize, region_type: char, visited: &mut HashSet<(usize, usize)>, ) -> Polygon {
    let mut queue = VecDeque::new();
    let mut coordinates = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut area = 0;
    let mut perimeter = 0;

    queue.push_back((start_i, start_j));
    while let Some((i, j)) = queue.pop_front() {
        if visited.contains(&(i, j)) || matrix[i][j] != region_type {
            continue;
        }
        visited.insert((i, j));
        let current_coord = Pos { x: i, y: j };
        coordinates.insert(current_coord);
        area += 1;

        for (dx, dy) in &directions {
            let new_i = (i as isize + dx) as usize;
            let new_j = (j as isize + dy) as usize;

            if new_i < matrix.len() && new_j < matrix[0].len() {
                if matrix[new_i][new_j] != region_type {
                    perimeter += 1;
                } else if !visited.contains(&(new_i, new_j)) {
                    queue.push_back((new_i, new_j));
                }
            } else {
                perimeter += 1;
            }
        }
    }

    Polygon {
        coordinates,
        area,
        perimeter,
    }
}

fn parse(input: &str) -> Vec<Polygon> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut polygons: Vec<Polygon> = Vec::new();
    let mut visited = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !visited.contains(&(i, j)) {
                let region_type = matrix[i][j];
                let polygon = flood_fill(&matrix, i, j, region_type, &mut visited);
                if !polygon.coordinates.is_empty() {
                    polygons.push(polygon);
                }
            }
        }
    }

    polygons
}

pub fn part_one(input: &str) -> Option<i32> {
    let polygons = parse(input);

    Some(polygons.iter().map(
        |p| p.area * p.perimeter
    ).sum())
}

fn get_neighbours(pos: Pos, dirs: Vec<(i32, i32)>) -> Vec<Pos> {
    dirs.iter().map(
        |(i, j)| Pos { x: (pos.x as i32 + i) as usize, y: (pos.y as i32 + j) as usize }
    ).collect()
}

fn get_neighbour(pos: Pos, (i, j): (i32, i32)) -> Pos {
    Pos { x: (pos.x as i32 + i) as usize, y: (pos.y as i32 + j) as usize }
}

fn get_corner_count(coordinates: &HashSet<Pos>) -> i32 {
    let mut corner_count = 0;

    for coordinate in coordinates {
        let empty_count = get_neighbours(*coordinate, vec![(-1, 0), (1, 0), (0, -1), (0, 1)])
            .iter()
            .filter(|n| !coordinates.contains(n))
            .count();

        corner_count += match empty_count {
            4 => 4, // single point inside the polygon
            3 => 2, // end of a one block wide section
            2 => {
                // one block wide section
                if coordinates.contains(&get_neighbour(*coordinate, (1, 0))) && coordinates.contains(&get_neighbour(*coordinate, (-1, 0))) ||
                    coordinates.contains(&get_neighbour(*coordinate, (0, 1))) && coordinates.contains(&get_neighbour(*coordinate, (0, -1))) {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        };

        // Inner corners:
        if !coordinates.contains(&get_neighbour(*coordinate, (-1,1))) &&
            coordinates.contains(&get_neighbour(*coordinate, (-1,0))) &&
            coordinates.contains(&get_neighbour(*coordinate, (0,1))) {
            corner_count += 1;
        }
        if !coordinates.contains(&get_neighbour(*coordinate, (1,1))) &&
            coordinates.contains(&get_neighbour(*coordinate, (1,0))) &&
            coordinates.contains(&get_neighbour(*coordinate, (0,1))) {
            corner_count += 1;
        }
        if !coordinates.contains(&get_neighbour(*coordinate, (-1,-1))) &&
            coordinates.contains(&get_neighbour(*coordinate, (-1,0))) &&
            coordinates.contains(&get_neighbour(*coordinate, (0,-1))) {
            corner_count += 1;
        }
        if !coordinates.contains(&get_neighbour(*coordinate, (1,-1))) &&
            coordinates.contains(&get_neighbour(*coordinate, (1,0)))  &&
            coordinates.contains(&get_neighbour(*coordinate, (0,-1))) {
            corner_count += 1;
        }
    }

    corner_count
}

pub fn part_two(input: &str) -> Option<i32> {
    let polygons = parse(input);

    Some(polygons.iter().map(
        |p| get_corner_count(&p.coordinates)  * p.area
    ).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
