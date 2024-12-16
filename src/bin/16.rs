use std::convert::Infallible;

use advent_of_code::solution;
use petgraph::graph::{DiGraph, Graph, NodeIndex};
use rustworkx_core::{self, distancemap::DistanceMap, shortest_path};
use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(16);


type Node = ((isize, isize), (isize, isize));

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
pub struct Maze {
    start: NodeIndex,
    end: NodeIndex,
    paths: Graph<((isize, isize), (isize, isize)), usize>,
}

impl Maze {

    fn rotate(dir: (isize, isize)) -> (isize, isize){

        match dir {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => !unreachable!(),
        }
    }

    // we use petgraph to make a graph network from the edges.
    fn from_input(input: &str) -> Maze {
    
        let mut paths= DiGraph::<Node, _>::new();
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        let mut nodes_to_indices = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {continue;},
                    'S' => {
                        start_pos = (y as isize, x as isize);
                        for dir in DIRECTIONS {
                            let node = ((y as isize, x as isize), dir);
                            nodes_to_indices.insert(node, paths.add_node(node));
                        };
                    },
                    'E' => {
                        end_pos = (y as isize, x as isize);
                        for dir in DIRECTIONS {
                            let node = ((y as isize, x as isize), dir);
                            nodes_to_indices.insert(node, paths.add_node(node));
                        };
                    },

                    // now check connected and add edges
                    '.' => {
                        for dir in DIRECTIONS {
                            let node = ((y as isize, x as isize), dir);
                            nodes_to_indices.insert(node, paths.add_node(node));
                        };
                    },
                    _ => !unreachable!(),
                };
            }
        }

        for ((pos, dir), idx) in nodes_to_indices.iter() {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(next_idx) = nodes_to_indices.get(&(next_pos, *dir)) {
                paths.add_edge(*idx, *next_idx, 1);
            }
            for new_dir in [Maze::rotate(*dir), Maze::rotate(Maze::rotate(Maze::rotate(*dir)))] {
                let next_idx = nodes_to_indices.get(&(*pos, new_dir)).unwrap();
                paths.add_edge(*idx, *next_idx, 1000);
            }
        }
        let end = paths.add_node(((-1, -1), (-1, -1)));
        for dir in DIRECTIONS {
            let idx = nodes_to_indices.get(&(end_pos, dir));
            match idx {
                Some(idx) => {paths.add_edge(*idx, end, 0);},
                None => ()
            }
    
            
        }
        let start = nodes_to_indices.get(&(start_pos, (0, 1))).unwrap();


        Maze { start: *start, end: end, paths: paths }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::from_input(input);
    let solution: rustworkx_core::dictmap::DictMap<petgraph::graph::NodeIndex, usize> =
    shortest_path::dijkstra(
        &maze.paths,
        maze.start,
        Some(maze.end),
        |e| Ok::<usize, Infallible>(*e.weight()),
        None,
    )
    .unwrap();
    let result = solution.get_item(maze.end).unwrap();
    // println!("part1 {:?}", result);
    Some(*result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::from_input(input);
    let solution = shortest_path::all_shortest_paths(&maze.paths, maze.start, maze.end, |e| {
        Ok::<usize, Infallible>(*e.weight())
    })
    .unwrap();
    let mut best_spots = HashSet::new();
    for path in solution {
        // println!("{:?}", path);
        for idx in &path[0..path.len() - 1] {
            best_spots.insert(maze.paths.node_weight(*idx).unwrap().0);
        }
    }
    let result = best_spots.len() as u32;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
