use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;
use std::ops::{Add, Sub};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::West,
    Direction::North,
    Direction::East,
    Direction::South,
];

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

    pub fn neighbor_in_grid(&self, direction: Direction, grid: &Vec<Vec<char>>) -> Option<(Pos, char)> {
        let neighbor = self.neighbor(direction);
        let (x, y ) = (neighbor.x as usize, neighbor.y as usize);

        if x < 0 || x >= grid.len() || y < 0 || y >= grid[0].len() {
            return None;
        }

        Some((neighbor, grid[x][y]))
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

pub fn parse_grid(input: &str) -> Vec<Vec<char>>
{
    input.lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
        })
        .collect()
}

pub fn get_value_from_grid<T>(grid: &Vec<Vec<T>>, x: usize, y: usize) -> Option<&T> {
    if x < grid.len() && y < grid[0].len() {
        Some(&grid[x][y])
    } else {
        None
    }
}

pub fn get_value_from_grid_pos<T>(grid: &Vec<Vec<T>>, pos: Pos) -> Option<&T> {
    let (x, y ) = (pos.x as usize, pos.y as usize);
    if x < grid.len() && y < grid[0].len() {
        Some(&grid[x][y])
    } else {
        None
    }
}

pub struct Graph<N> {
    pub nodes: HashMap<N, HashSet<N>>,
}

impl<N: Eq + Hash + Clone> Graph<N> {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes.entry(node).or_insert_with(HashSet::new);
    }

    pub fn has_node(&self, node: &N) -> bool {
        self.nodes.get(node).is_some()
    }

    pub fn add_edge(&mut self, node1: N, node2: N) {
        if !self.has_edge(&node1, &node2) {
            self.nodes.entry(node1.clone()).or_insert_with(HashSet::new);
            self.nodes.entry(node2.clone()).or_insert_with(HashSet::new);

            self.nodes.get_mut(&node1).unwrap().insert(node2.clone());
            self.nodes.get_mut(&node2).unwrap().insert(node1);
        }
    }

    pub fn has_edge(&self, node1: &N, node2: &N) -> bool {
        if let Some(neighbors) = self.neighbors(node1) {
            neighbors.contains(node2)
        } else {
            false
        }
    }

    pub fn neighbors(&self, node: &N) -> Option<&HashSet<N>> {
        self.nodes.get(node)
    }
}

fn bron_kerbosch<'a>(
    g: &HashMap<&'a str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r.clone());
        }
        return;
    }
    while let Some(n) = p.iter().copied().next() {
        let neighbours = &g[n];
        let p2 = p.intersection(neighbours).copied().collect();
        let x2 = x.intersection(neighbours).copied().collect();
        r.insert(n);
        bron_kerbosch(g, r, p2, x2, cliques);
        r.remove(n);
        p.remove(n);
        x.insert(n);
    }
}
