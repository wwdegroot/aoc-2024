advent_of_code::solution!(18);

use std::{convert::Infallible};

use hashbrown::{HashMap, HashSet};
use petgraph::graph::{DiGraph, Graph, NodeIndex};
use rustworkx_core::{self, distancemap::DistanceMap, shortest_path};

type Node = (isize, isize);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct Grid {
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct MemSpace {
    start: NodeIndex,
    end: NodeIndex,
    graph: Graph<(isize, isize), usize>,
    nodes_indeces: HashMap<(isize, isize), NodeIndex>
}

pub fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|l| {
            let (y, x) = l.split_once(",").unwrap();
            (y.parse::<isize>().unwrap(), x.parse::<isize>().unwrap())
        })
        .collect()
}

pub fn simulate_byte_drops(droplist: &Vec<(isize, isize)>, ns: usize, grid: &Grid) -> MemSpace {
    let mut graph = DiGraph::<Node, _>::new();
    let start_pos = (0, 0);
    let end_pos = ((grid.width - 1) as isize, (grid.height - 1) as isize);
    let dropping: HashSet<(isize, isize)> = droplist
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if i < ns { Some(*c) } else { None })
        .collect();
    let mut nodes_to_indices = HashMap::new();

    // add nodes
    for y in 0..grid.height {
        for x in 0..grid.width {
            let node = (y as isize, x as isize);
            if !dropping.contains(&node) {
                // println!("'.' found so inserting");
                nodes_to_indices.insert(node, graph.add_node(node));
            } else {
                // println!("'#' byte drop found")
            }
        }
    }
    // add edges
    for (pos, idx) in nodes_to_indices.iter() {
        for dir in DIRECTIONS {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(next_idx) = nodes_to_indices.get(&(next_pos)) {
                graph.add_edge(*idx, *next_idx, 1);
            }
        }
    }
    let start_idx = nodes_to_indices.get(&(start_pos)).unwrap();
    let end_idx = nodes_to_indices.get(&(end_pos)).unwrap();
    MemSpace {
        start: *start_idx,
        end: *end_idx,
        graph: graph,
        nodes_indeces: nodes_to_indices,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let byte_drops = parse_input(input);
    let grid = Grid {
        width: 71,
        height: 71,
    };
    let memspace = simulate_byte_drops(&byte_drops, 1024, &grid);
    let solution: rustworkx_core::dictmap::DictMap<petgraph::graph::NodeIndex, usize> =
        shortest_path::dijkstra(
            &memspace.graph,
            memspace.start,
            Some(memspace.end),
            |e| Ok::<usize, Infallible>(*e.weight()),
            None,
        )
        .unwrap();
    let result = solution.get_item(memspace.end).unwrap();
    Some(*result as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let byte_drops = parse_input(input);
    let grid = Grid {
        width: 71,
        height: 71,
    };
    let mut byte_drop_idx: usize = 1024;
    
    for i in byte_drop_idx..byte_drops.len() {
        // let node_to_remove = memspace.nodes_indeces.get(&byte_drops[i]).unwrap();
        // memspace.graph.remove_node(*node_to_remove);
        let memspace = simulate_byte_drops(&byte_drops, i, &grid);
        let solution: rustworkx_core::dictmap::DictMap<petgraph::graph::NodeIndex, usize> =
        shortest_path::dijkstra(
            &memspace.graph,
            memspace.start,
            Some(memspace.end),
            |e| Ok::<usize, Infallible>(*e.weight()),
            None,
        )
        .unwrap();
        if let Some(_result) = solution.get_item(memspace.end) {
            continue;
        } else {
            // no longer a path to the exit
            byte_drop_idx = i -1;
            break;
        }
    }
    let result = byte_drops[byte_drop_idx];

    Some(format!("{},{}", result.0, result.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let byte_drops = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let grid = Grid {
            width: 7,
            height: 7,
        };
        let memspace = simulate_byte_drops(&byte_drops, 12, &grid);
        // println!("{:#?}", memspace);
        let solution: rustworkx_core::dictmap::DictMap<petgraph::graph::NodeIndex, usize> =
            shortest_path::dijkstra(
                &memspace.graph,
                memspace.start,
                Some(memspace.end),
                |_| Ok::<usize, Infallible>(1),
                None,
            )
            .unwrap();
        let result = solution.get_item(memspace.end).unwrap();
        assert_eq!(*result, 22);
    }

    #[test]
    fn test_part_two() {
        let byte_drops = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let grid = Grid {
            width: 7,
            height: 7,
        };
        let mut byte_drop_idx: usize = 12;
        for i in 12..byte_drops.len() {
            // println!("i= {} ,byte={:?}",i, byte_drops[i]);
            let memspace = simulate_byte_drops(&byte_drops, i, &grid);
            let solution: rustworkx_core::dictmap::DictMap<petgraph::graph::NodeIndex, usize> =
            shortest_path::dijkstra(
                &memspace.graph,
                memspace.start,
                Some(memspace.end),
                |e| Ok::<usize, Infallible>(*e.weight()),
                None,
            )
            .unwrap();
            if let Some(_result) = solution.get_item(memspace.end) {
                // println!("{_result}");
                continue;
            } else {
                // no longer a path to the exit
                byte_drop_idx = i - 1;
                break;
            }
        }
        let result = byte_drops[byte_drop_idx];
        assert_eq!(result, (6,1));
    }
}
