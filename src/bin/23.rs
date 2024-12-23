#![allow(dead_code)]
#[path = "utils/grid_utils.rs"] mod utils;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::utils::{Graph};

advent_of_code::solution!(23);

fn parse(input: &str) -> Graph<&str> {
    let mut graph = Graph::new();

    input.lines().for_each(
        |l| {
            let mut split = l.split('-');
            graph.add_edge(split.next().unwrap(), split.next().unwrap())
        }
    );

    graph
}

fn has_t(node1: &str, node2: &str, node3:  &str) -> bool {
    node1.starts_with("t") || node2.starts_with("t") || node3.starts_with("t")
}

fn find_triangles(graph: Graph<&str>) -> Vec<Vec<&str>> {
    let mut triangles: Vec<Vec<&str>> = Vec::new();

    for node in graph.nodes.keys() {
        if let neighbors = graph.neighbors(node).unwrap().iter().collect_vec() {
            for i in 0..neighbors.len() {
                for j in i + 1..neighbors.len() {
                    let neighbor1 = &neighbors[i];
                    let neighbor2 = &neighbors[j];

                    if graph.has_edge(neighbor1, neighbor2) && has_t(node, neighbor1, neighbor2) {
                        let mut triangle = vec![node.clone(), neighbor1.clone(), neighbor2.clone()];
                        triangle.sort();
                        if !triangles.contains(&triangle) {
                            triangles.push(triangle);
                        }
                    }
                }
            }
        }
    }

    triangles
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse(input);

    let mut triangles = find_triangles(graph);
    triangles.sort();
    Some(triangles.len())
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


fn parse2(input: &str) -> Graph<&str> {
    let mut graph = Graph::new();

    input.lines().for_each(
        |l| {
            let mut split = l.split('-');
            graph.add_edge(split.next().unwrap(), split.next().unwrap())
        }
    );

    graph
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse2(input);

    let mut cliques = Vec::new();
    bron_kerbosch(&graph.nodes, &mut HashSet::new(), graph.nodes.keys().copied().collect(), HashSet::new(), &mut cliques);

    let biggest_team = cliques.iter().max_by_key(|c| c.len()).unwrap().iter().sorted().join(",");

    Some(biggest_team)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
