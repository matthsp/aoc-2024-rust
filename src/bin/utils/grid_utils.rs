use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Debug;
use std::ops::{Add, Sub};


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

// Implement Add trait for Pos
impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}

// Implement Sub trait for Pos
impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
        }
    }
}

impl Pos {
    // Constructor method to create a new Pos
    pub fn new(x: isize, y: isize) -> Self { Pos { x, y } }

    pub fn neighbor(&self, direction: Direction) -> Pos {
        let (new_x, new_y) = match direction {
            Direction::North => (self.x.wrapping_sub(1), self.y),
            Direction::NorthEast => (self.x.wrapping_sub(1), self.y + 1),
            Direction::East => (self.x, self.y + 1),
            Direction::SouthEast => (self.x + 1, self.y + 1),
            Direction::South => (self.x + 1, self.y),
            Direction::SouthWest => (self.x + 1, self.y.wrapping_sub(1)),
            Direction::West => (self.x, self.y.wrapping_sub(1)),
            Direction::NorthWest => (self.x.wrapping_sub(1), self.y.wrapping_sub(1)),
        };

        Pos { x: new_x, y: new_y }
    }

    pub fn neighbors(&self, directions: &[Direction]) -> Vec<Pos> {
        directions.iter()
            .map(|&direction| self.neighbor(direction))
            .collect()
    }
}

#[derive(Debug)]
pub struct Edge {
    pub start: Pos,
    pub end: Pos,
}

impl Edge {
    // Constructor method to create a new Edge
    pub fn new(start: Pos, end: Pos) -> Self { Edge { start, end } }

    pub fn contains(&self, pos: &Pos) -> bool {
        let Edge { start, end } = self;

        let is_between = |val, min, max| val >= min && val <= max;

        if start.x == end.x && start.x == pos.x {
            // Vertical edge
            let (min_y, max_y) = if start.y < end.y { (start.y, end.y) } else { (end.y, start.y) };
            return is_between(pos.y, min_y, max_y);
        }
        if start.y == end.y && start.y == pos.y {
            // Horizontal edge
            let (min_x, max_x) = if start.x < end.x { (start.x, end.x) } else { (end.x, start.x) };
            return is_between(pos.x, min_x, max_x);
        }

        // Diagonal edge
        let cross_product = (pos.y as isize - start.y as isize) * (end.x as isize - start.x as isize)
            - (pos.x as isize - start.x as isize) * (end.y as isize - start.y as isize);

        if cross_product != 0 {
            return false;
        }

        let (min_x, max_x) = if start.x < end.x { (start.x, end.x) } else { (end.x, start.x) };
        let (min_y, max_y) = if start.y < end.y { (start.y, end.y) } else { (end.y, start.y) };
        is_between(pos.x, min_x, max_x) && is_between(pos.y, min_y, max_y)
    }

    pub fn length(&self) -> f64 {
        let dx = (self.end.x as isize - self.start.x as isize).pow(2);
        let dy = (self.end.y as isize - self.start.y as isize).pow(2);
        ((dx + dy) as f64).sqrt()
    }
}

pub fn parse_grid<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

fn get_value_from_grid<T>(grid: &Vec<Vec<T>>, x: usize, y: usize) -> Option<&T> {
    if x < grid.len() && y < grid[0].len() {
        Some(&grid[x][y])
    } else {
        None
    }
}

pub struct Graph {
    pub nodes: HashMap<Pos, Vec<Pos>>,
}

impl Graph {
    fn add_node(&mut self, pos: Pos) {
        self.nodes.entry(pos).or_insert(Vec::new());
    }

    fn has_node(&self, pos: &Pos) -> bool {
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
