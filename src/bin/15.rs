advent_of_code::solution!(15);

use hashbrown::HashMap;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Surface {
    Wall,
    Box,
    Tile
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct WareHouse {
    map: HashMap<(i32, i32), Surface>,
    robot: (i32, i32),
    rows: i32,
    cols: i32,
}

impl WareHouse {

    fn new(input: &str) -> WareHouse {
        let mut robot: (i32, i32) = (0, 0);
        let mut rows: i32 = 0;
        let mut cols: i32 = 0;
        let map = input.lines().enumerate().fold(HashMap::new(), |mut acc, (row, line)|{
            rows = row as i32;
            line.chars().enumerate().for_each(|(col ,c)|{
                if cols < col.try_into().unwrap() { cols = col as i32};
                match c {
                    '#' => {acc.insert((row as i32, col as i32), Surface::Wall);},
                    'O' => {acc.insert((row as i32, col as i32), Surface::Box);},
                    '@' => {
                        robot = (row as i32, col as i32);
                        acc.insert((row as i32, col as i32), Surface::Tile);
                    },
                    _ => {acc.insert((row as i32, col as i32), Surface::Tile);},
                }
            });
            acc
        });
        WareHouse{ map , robot, rows, cols }
    }

    fn next_tile(&self, position: (i32, i32), command: &Direction) -> (i32, i32) {
        let next_tile = match command {
            &Direction::Down => (position.0 +1, position.1),
            &Direction::Up => (position.0 - 1, position.1),
            &Direction::Left => (position.0, position.1  - 1),
            &Direction::Right => (position.0, position.1 + 1),
        };
        next_tile
    }

    fn next_boxes(&self, position: (i32, i32), command: &Direction) -> Option<Vec<(i32, i32)>> {
        let mut next_boxes: Vec<(i32, i32)> = vec![];
        let mut legal: bool = false;
        match command {
            &Direction::Down => {
                for r in position.0..self.rows {
                    match self.map.get(&(r, position.1)) {
                        Some(&Surface::Box) => {next_boxes.push((r, position.1));},
                        Some(&Surface::Tile) => {legal = true; next_boxes.push((r, position.1)); break;},
                        Some(&Surface::Wall) => {break;}
                        None => !unreachable!(),
                    }
                }
            },
            &Direction::Up => {
                for r in (0..position.0).rev() {
                    match self.map.get(&(r, position.1)) {
                        Some(&Surface::Box) => {next_boxes.push((r, position.1));},
                        Some(&Surface::Tile) => {legal = true; next_boxes.push((r, position.1)); break;},
                        Some(&Surface::Wall) => {break;}
                        None => !unreachable!(),
                    }
                }
            },
            &Direction::Left => {
                for c in (0..position.1).rev() {
                    match self.map.get(&(position.0, c)) {
                        Some(&Surface::Box) => {next_boxes.push((position.0, c));},
                        Some(&Surface::Tile) => {legal = true; next_boxes.push((position.0, c)); break;},
                        Some(&Surface::Wall) => {break;}
                        None => !unreachable!(),
                    }
                }
            },
            &Direction::Right => {
                for c in position.1..self.cols {
                    match self.map.get(&(position.0, c)) {
                        Some(&Surface::Box) => {next_boxes.push((position.0, c));},
                        Some(&Surface::Tile) => {legal = true; next_boxes.push((position.0, c)); break;},
                        Some(&Surface::Wall) => {break;}
                        None => !unreachable!(),
                    }
                }
            },
        };
        if legal {
            Some(next_boxes)
        } else {
            None
        }
        
    }

    fn move_robot(&mut self, commands: Vec<Direction> ) {
        for command in commands {
            let dir = self.next_tile(self.robot, &command);
            // now decide what the next legal move and board state is.
            match self.map.get(&dir).unwrap() {
                // Nothing special here just move robot
                &Surface::Tile => {
                    self.robot = dir;
                },
                // Now check tiles behind the box and see if there is an empty tile.
                &Surface::Box => {
                    let next_boxes = self.next_boxes(dir, &command);
                    // We found boxes to move
                    if next_boxes.is_some() {
                        // update all returned tiles to Box
                        for b in next_boxes.unwrap() {
                            *self.map.entry(b).or_insert(Surface::Box) = Surface::Box;
                        }
                        // set dir to Tile
                        *self.map.entry(dir).or_insert(Surface::Tile) = Surface::Tile;
                        // move robot
                        self.robot = dir;
                    }
                },
                // Can't move, so do nothing
                &Surface::Wall => {

                },
            }
        }
    }

}

pub fn parse_commands(input: &str) -> Vec<Direction> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => !unreachable!(),
            }
        }).collect::<Vec<Direction>>()
    }).flatten().collect()
}


pub fn score_gps(warehouse: WareHouse) -> i32 {
    warehouse.map.iter().map(|(k,v)|{
        if v == &Surface::Box {
            100 * k.0 + k.1
        } else {
            0
        }
    }).sum()
}


pub fn part_one(input: &str) -> Option<i32> {
    let (input_map, input_commands) = input.split_once("\n\n").expect("Expected input to be seperated by empty newline");
    let mut warehouse = WareHouse::new(input_map);
    // println!("{:#?}", warehouse);
    let commands: Vec<Direction> = parse_commands(input_commands);
    // println!("{:#?}", commands);
    warehouse.move_robot(commands);
    let result = score_gps(warehouse);
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
