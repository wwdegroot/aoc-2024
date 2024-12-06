advent_of_code::solution!(6);

use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Surface {
    Object,
    Tile,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_input(input: &str) -> ((i32, i32), HashMap<(i32, i32), Surface>) {
    let (guard, map) = input.lines().enumerate().into_iter().fold(
        ((0, 0), HashMap::new()),
        |(mut guard, mut acc), (row, line)| {
            line.chars().enumerate().for_each(|(col, char)| {
                let r: i32 = row.try_into().unwrap();
                let c: i32 = col.try_into().unwrap();
                match char {
                    '.' => acc.insert((r, c), Surface::Tile),
                    '#' => acc.insert((r, c), Surface::Object),
                    '^' => {
                        guard = (r, c);
                        acc.insert((r, c), Surface::Tile)
                    }
                    _ => !unreachable!(),
                };
            });

            (guard, acc)
        },
    );
    (guard, map)
}

pub fn match_direction(position: (i32, i32), direction: Directions) -> (i32, i32) {
    let (row_change, col_change) = match direction {
        Directions::Up => (1_i32, 0_i32),
        Directions::Down => (-1_i32, 0_i32),
        Directions::Left => (0_i32, 1_i32),
        Directions::Right => (0_i32, -1_i32),
    };
    (position.0 - row_change, position.1 - col_change)
}


pub fn solve_loop_path(    guard: (i32, i32),
mut direction: Directions,
map: &HashMap<(i32, i32), Surface>) -> bool {
    let mut loop_detected: bool = false;
    let mut iteration_count: i32 = 0;
    let mut position = guard.clone();

    loop {
        let new_position = match_direction(position, direction);
        //println!("next= {:?}", new_position);
        let next_tile = map.get(&new_position);
        match next_tile {
            None => break,
            Some(Surface::Object) => {
                direction = match direction {
                    Directions::Up => Directions::Right,
                    Directions::Down => Directions::Left,
                    Directions::Left => Directions::Up,
                    Directions::Right => Directions::Down,
                };
            }
            Some(Surface::Tile) => position = new_position,
        }
        //println!("next position={:?}, direction={:?}", position, direction);

        // count the iterations and assume its a loop when the number is reached
        if iteration_count > 8000 {
            loop_detected = true;
            break;
        }

        iteration_count += 1;
    }
    loop_detected
}

pub fn solve_path(
    guard: (i32, i32),
    mut direction: Directions,
    map: &HashMap<(i32, i32), Surface>,
) -> (HashMap<(i32, i32), Directions>, bool, Vec<(i32, i32)>) {
    // this runs forever for the situation when the guard walks in a circle
    // of course part 2 wants to get into infinite loops, so we need to detect this case.
    // we do this by checking when the direction and position match the already visited direction and position
    let mut loop_detected: bool = false;
    let mut iteration_count: i32 = 0;
    let mut position = guard.clone();
    let mut visited: HashMap<(i32, i32), Directions> = HashMap::new();
    let mut visited_tiles: Vec<(i32, i32)> = Vec::with_capacity(4000);
    // println!("start={:?}", guard);
    loop {
        visited.insert(position, direction);
        visited_tiles.push(position);
        let new_position = match_direction(position, direction);
        //println!("next= {:?}", new_position);
        let next_tile = map.get(&new_position);
        match next_tile {
            None => break,
            Some(Surface::Object) => {
                direction = match direction {
                    Directions::Up => Directions::Right,
                    Directions::Down => Directions::Left,
                    Directions::Left => Directions::Up,
                    Directions::Right => Directions::Down,
                };
            }
            Some(Surface::Tile) => position = new_position,
        }
        //println!("next position={:?}, direction={:?}", position, direction);

        // count the iterations and assume its a loop when the number is reached
        if iteration_count > 8000 {
            loop_detected = true;
            break;
        }

        iteration_count += 1;
    }
    (visited, loop_detected, visited_tiles)
}

pub fn solve_obstructed_path(
    guard: (i32, i32),
    direction: Directions,
    map: HashMap<(i32, i32), Surface>,
) -> usize {
    let (_, _, visited_tiles) = solve_path(guard, direction, &map);
    let unique_tiles: Vec<(i32, i32)> = visited_tiles.into_iter().unique().collect();
    let count: usize = unique_tiles.into_par_iter().map(|object|{
                // do not place in starting position guard
                if object == guard {
                    return 0
                }
                // solve_path on map with added object
                let mut map_clone = map.clone();
                map_clone.insert(object, Surface::Object);
                if solve_loop_path(guard, direction, &map_clone) {
                    return 1
                } else {
                    return 0
                }
    }).sum();
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let (guard, map) = parse_input(input);
    let direction = Directions::Up;
    let (result, _, _) = solve_path(guard, direction, &map);
    Some(result.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (guard, map) = parse_input(input);
    let direction = Directions::Up;
    let result = solve_obstructed_path(guard, direction, map);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
