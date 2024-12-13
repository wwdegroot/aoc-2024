advent_of_code::solution!(12);

use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

pub fn parse_input_grids(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Breadth first search implementation
pub fn find_regions(plots: Vec<Vec<char>>) -> Vec<Vec<(i32, i32)>> {
    // Rows and columns of the map
    let rows = plots.len() as i32;
    let cols = plots[0].len() as i32;

    // keep track of visited cells
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // all unique regions
    let mut regions: Vec<Vec<(i32, i32)>> = vec![];
    // coordinates available in the grid
    let mut coorinates: Vec<(i32, i32)> = vec![];
    for r in 0..rows {
        for c in 0..cols {
            coorinates.push((r.try_into().unwrap(), c.try_into().unwrap()));
        }
    }

    for grid in coorinates.iter() {
        // skip visited cells
        if visited.contains(grid) {
            continue;
        }

        let pos = grid.clone();

        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut region: Vec<(i32, i32)> = vec![];
        // update state
        let mut last_seen = plots[pos.0 as usize][pos.1 as usize];
        visited.insert(pos);
        region.push(pos);
        q.push_back(pos);
        // no infinite loops here
        let mut i = 0;
        while !q.is_empty() && i < 100_000 {
            // Dequeue the front node
            let front = q.pop_front().unwrap();
            let (r, c) = front.clone();
            // Check if the adjacent cells have the same character
            // check below
            if r + 1 < rows
                && plots[r as usize + 1][c as usize] == last_seen
                && !visited.contains(&(r + 1, c))
            {
                visited.insert((r + 1, c));
                region.push((r + 1, c));
                q.push_back((r + 1, c));
            }

            // check above
            if r > 0
                && plots[r as usize - 1][c as usize] == last_seen
                && !visited.contains(&(r - 1, c))
            {
                visited.insert((r - 1, c));
                region.push((r - 1, c));
                q.push_back((r - 1, c));
            }

            // check right
            if c + 1 < cols
                && plots[r as usize][c as usize + 1] == last_seen
                && !visited.contains(&(r, c + 1))
            {
                visited.insert((r, c + 1));
                region.push((r, c + 1));
                q.push_back((r, c + 1));
            }

            // check left
            if c > 0
                && plots[r as usize][c as usize - 1] == last_seen
                && !visited.contains(&(r, c - 1))
            {
                visited.insert((r, c - 1));
                region.push((r, c - 1));
                q.push_back((r, c - 1));
            }
            i += 1;
        }
        regions.push(region);
    }
    regions
}

pub fn calculate_fencing(regions: Vec<Vec<(i32, i32)>>) -> usize {
    regions
        .iter()
        .map(|region| {
            let area: usize = region.len().try_into().unwrap();
            let mut fencing: usize = 0;
            for cell in region {
                let mut fences = 4;
                for (dr, dc) in [(0_i32, 1_i32), (1, 0), (0, -1), (1, 0)] {
                    if region.contains(&(cell.0 + dr, cell.1 + dc)) {
                        fences -= 1;
                    }
                }
                fencing += fences
            }
            fencing * area
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse_input_grids(input);
    let regions = find_regions(grids);
    let result = calculate_fencing(regions);
    // println!("{:#?}", result);
    Some(result)
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
