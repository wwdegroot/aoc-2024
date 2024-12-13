advent_of_code::solution!(12);

use hashbrown::HashMap;
use std::collections::VecDeque;

pub struct Map {
    width: usize,
    heigth: usize,
    plots: Vec<Vec<char>>,
    visited: HashMap<(usize, usize), bool>,
}

impl Map {
    fn regions(&mut self) {}
}

pub fn does_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    let mut overlap = true;
    if a_start >= b_end {
        overlap = false
    }
    if b_start >= a_end {
        overlap = false
    }
    overlap
}

pub fn parse_input_grids(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Breadth first search implementation
pub fn find_regions(plots: Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    // Rows and columns of the map
    let rows = plots.len();
    let cols = plots[0].len();

    // keep track of visited cells
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    for r in 0..rows {
        for c in 0..cols {
            visited.insert((r,c), false);
        }
    }


    // all unique regions
    let mut regions: Vec<Vec<(usize, usize)>> = vec![];

    for pos in visited.keys() { 
        // skip visited cells
        if visited.contains_key(pos){
            continue
        }

        let mut q = VecDeque::new();
        let mut region = vec![];
        // update state
        let mut last_seen = plots[pos.0][pos.1];
        visited.insert(*pos, true);
        region.push(pos);
        q.push_back(pos);
        while !q.is_empty() {
            // Dequeue the front node
            let (r, c) = q.front().unwrap();

            // Check if the adjacent cells have the same character
            // check below
            if r + 1 < rows && plots[r + 1][*c] == last_seen && !visited.contains_key(&(r + 1, c)) {
                visited.insert((r + 1, *c), true);
                region.push((r + 1, c));
                q.push_back((r + 1, c));
            }

            // check above
            if r - 1 >= 0 && plots[r - 1][c] == last_seen && !visited.contains_key((r + 1, c)) {
                visited[(r - 1, c)] = True
                region.append((r - 1, c))
                q.append((r - 1, c))
            }

            // check right
            if c + 1 < cols and plots[r][c + 1] == last_seen and not visited[(r, c + 1)] {
                visited[(r, c + 1)] = True
                region.append((r, c + 1))
                q.append((r, c + 1))
            }

            // check left
            if c - 1 >= 0 and plots[r][c - 1] == last_seen and not visited[(r, c - 1)]{
                visited[(r, c - 1)] = True
                region.append((r, c - 1))
                q.append((r, c - 1))
            }


            
        }
        regions.push(region);
    }
    regions
}

pub fn parse_input(input: &str) -> Map {
    let plots: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let heigth = plots.len();
    let width = plots[0].len();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut regions_by_row: Vec<Vec<RegionRow>> = Vec::with_capacity(width);

    for (row, line) in plots.iter().enumerate() {
        let mut regions: Vec<RegionRow> = vec![];
        // inital char plot
        let mut last_seen: char = plots[row][0];
        // inital row region
        let mut rowregion: RegionRow = RegionRow {
            letter: last_seen,
            row_index: row,
            start: 0,
            end: 0,
            nodes: vec![],
        };
        for (col, c) in line.iter().enumerate() {
            visited.insert((row, col), false);
            // If last seen char exists and is similar add to rowregion
            if &last_seen == c {
                rowregion.nodes.push(Plot {
                    coordinate: (row, col),
                });
                last_seen = *c;
            }
            // new rowregion
            if &last_seen != c {
                rowregion.end = col - 1;
                regions.push(rowregion);
                rowregion = RegionRow {
                    letter: *c,
                    start: col,
                    end: col,
                    row_index: row,
                    nodes: vec![Plot {
                        coordinate: (row, col),
                    }],
                };
                last_seen = *c;
            }
            // push last item to vec
            if heigth == row + 1 && width == col + 1 {
                rowregion.end = col;
                regions.push(rowregion);
                rowregion = RegionRow {
                    start: col,
                    end: col,
                    letter: *c,
                    row_index: row,
                    nodes: vec![],
                }
            }
        }
        regions_by_row.push(regions);
    }
    // make a map for each row containt the regionrow for chars
    // let mut regions_map: HashMap<usize, HashMap<char, Vec<RegionRow>>> = HashMap::new();
    // loop door rijen en kijk wanneer deze begint en eindigt en of deze overlapt met dezelfde rij eronder
    // Uiteindelijk regio's zonder overlap maken.
    let mut regions: Vec<Region> = vec![];
    for r in 0..heigth {
        if r == heigth - 1 {
            // dont run the last row
            break;
        }
        for rowregion in regions_by_row[r].iter() {
            // initial
            let mut region = Region {
                row_start: r,
                row_end: r,
                letter: rowregion.letter,
                nodes: rowregion.nodes.clone(),
            };
            for rowregion_next in regions_by_row[r + 1].iter() {
                if rowregion.letter == rowregion_next.letter {
                    // check if start end of regions overlap, if yes we have add to region
                    if does_overlap(
                        rowregion.start,
                        rowregion.end,
                        rowregion_next.start,
                        rowregion_next.end,
                    ) {
                        region.nodes.extend(rowregion_next.nodes.clone());
                    }
                }
            }
        }
    }
    println!("{:#?}", regions_by_row);
    Map {
        plots,
        width,
        heigth,
        visited,
    }
}

#[derive(Debug)]
pub struct Region {
    row_start: usize,
    row_end: usize,
    letter: char,
    nodes: Vec<Plot>,
}

#[derive(Debug, Clone)]
pub struct RegionRow {
    letter: char,
    start: usize,
    end: usize,
    row_index: usize,
    nodes: Vec<Plot>,
}

#[derive(Debug, Clone, Copy)]
pub struct Plot {
    coordinate: (usize, usize),
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
