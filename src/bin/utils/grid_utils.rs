use std::char;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
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

impl Direction {
    pub fn get_axis(&self) -> usize {
        match self {
            Self::North | Self::South => 0,
            Self::NorthEast | Direction::SouthWest => 2,
            Self::West | Self::East => 1,
            Self::SouthEast | Direction::NorthWest => 3,
        }
    }
}

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::West,
    Direction::North,
    Direction::East,
    Direction::South,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
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

pub fn is_in_boundaries(grid: &Vec<Vec<char>>, pos: Pos) -> bool {
    let (x, y ) = (pos.x as usize, pos.y as usize);
    x < 0 || x >= grid.len() || y < 0 || y >= grid[0].len()
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

pub struct Maze {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
    height: usize,
    width: usize,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);

        for (row, line) in value.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                if char == 'S' {
                    start = Pos::new(row as isize, col as isize);
                }
                if char == 'E' {
                    end = Pos::new(row as isize, col as isize)
                }

                grid_row.push(char);
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            start,
            end,
            height,
            width,
        }
    }
}

impl Maze {
    pub fn dijkstra(&self) -> usize {
        let mut min_cost = usize::MAX;
        let mut to_visit = vec![vec![usize::MAX; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        to_visit[self.start.x as usize][self.start.y as usize] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            direction: Direction::East,
            cost: 0,
            history: None,
        }));
        while let Some(Reverse(Tile {
                                   position,
                                   direction,
                                   cost,
                                   history: _,
                               })) = prio.pop()
        {
            let (row, col) = (position.x as usize, position.y as usize);
            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }
            if cost > to_visit[row][col] || cost >= min_cost {
                continue;
            }

            for dir in CARDINAL_DIRECTIONS {
                let next_dir = dir;
                let next_pos = position.neighbor(next_dir);
                let (next_row, next_col) = (next_pos.x as usize, next_pos.y as usize);
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_cost += 1001;
                }
                if self.grid[next_row][next_col] == '#' {
                    continue;
                }

                if next_cost < to_visit[next_row][next_col] {
                    to_visit[next_row][next_col] = next_cost;
                    prio.push(Reverse(Tile {
                        position: Pos::new(next_row as isize, next_col as isize),
                        direction: next_dir,
                        cost: next_cost,
                        history: None,
                    }));
                }
            }
        }
        min_cost
    }
    pub fn dijkstra_with_backtrack(&self, min_cost: usize, direction: &Direction) -> usize {
        let mut to_visit = vec![vec![[min_cost, min_cost]; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        let mut tiles = HashSet::new();
        to_visit[self.start.x as usize][self.start.y as usize][direction.get_axis()] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            direction: Direction::East,
            cost: 0,
            history: Some(vec![]),
        }));
        while let Some(Reverse(Tile {
                                   position,
                                   direction,
                                   cost,
                                   history,
                               })) = prio.pop()
        {
            let mut history = history.unwrap();
            history.push(position);

            let (row, col) = (position.x as usize, position.y as usize);
            if cost > to_visit[row][col][direction.get_axis()] || cost > min_cost {
                continue;
            }

            if position == self.end {
                if cost == min_cost {
                    tiles.extend(history);
                }

                continue;
            }
            for dir in CARDINAL_DIRECTIONS {
                let next_dir = dir;
                let next_pos = position.neighbor(dir);
                let (next_row, next_col) = (next_pos.x as usize, next_pos.y as usize);
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_cost += 1001;
                }
                if self.grid[next_row][next_col] == '#' {
                    continue;
                }

                if next_cost <= to_visit[next_row][next_col][direction.get_axis()] {
                    to_visit[next_row][next_col][direction.get_axis()] = next_cost;
                    prio.push(Reverse(Tile {
                        position: Pos::new(next_row as isize, next_col as isize),
                        direction: next_dir,
                        cost: next_cost,
                        history: Some(history.clone()),
                    }));
                }
            }
        }

        tiles.len()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    position: Pos,
    direction: Direction,
    cost: usize,
    history: Option<Vec<Pos>>,
}
