advent_of_code::solution!(8);

use hashbrown::{hash_map::Entry, HashMap, HashSet};
use itertools::{enumerate, Itertools};

pub fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut row_max: i32 = 0;
    let mut col_max: i32 = 0;
    map = input.lines().enumerate().fold(map, |mut acc, (row, line)| {
        col_max = line.len() as i32;
        line.chars()
            .into_iter()
            .enumerate()
            .for_each(|(col, c)| match c {
                '.' => (),
                _ => match acc.entry(c) {
                    Entry::Vacant(acc) => {
                        acc.insert(vec![(row as i32, col as i32)]);
                    }
                    Entry::Occupied(mut acc) => {
                        acc.get_mut().push((row as i32, col as i32));
                    }
                },
            });
        row_max += 1;
        acc
    });
    (map, (row_max, col_max))
}

fn calculate_antinodes(antenne1: (i32, i32), antenne2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    (
        (
            antenne1.0 - (antenne2.0 - antenne1.0),
            antenne1.1 - (antenne2.1 - antenne1.1),
        ),
        (
            antenne2.0 - (antenne1.0 - antenne2.0),
            antenne2.1 - (antenne1.1 - antenne2.1),
        ),
    )
}

fn calculate_all_antinodes(
    antenne1: (i32, i32),
    antenne2: (i32, i32),
    col_max: i32,
    row_max: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    let (col1, row1) = antenne1;
    let (col2, row2) = antenne2;
    println!("Antenne1 = {:?}", antenne1);
    println!("Antenne2 = {:?}", antenne2);
    for r in 0..row_max {
        for c in 0..col_max {
            // calculate the r (row) and c (col) position where the difference is zero, this is a valid antinode location
            if (row1 * (col2 - r) + row2 * (r - col1) + c * (col1 - col2)) == 0 {
                println!("grid loction row={}, col={}", r, c);
                println!(
                    "{} * ({} - {}) + {} * ({} - {}) + {} * ({} - {}) = 0",
                    row1, col2, r, row2, r, col1, c, col1, col2
                );
                antinodes.insert((c, r));
            }
        }
    }

    antinodes
}

pub fn solve_antinodes(
    antennes: HashMap<char, Vec<(i32, i32)>>,
    col_max: i32,
    row_max: i32,
) -> u32 {
    let mut antinodes = HashSet::new();
    for (_c, locations) in antennes.iter() {
        for l in locations.iter().combinations(2) {
            let l1 = *l[0];
            let l2 = *l[1];

            let (nl1, nl2) = calculate_antinodes(l1, l2);

            if nl1.0 >= 0 && nl1.0 < row_max && nl1.1 >= 0 && nl1.1 < col_max {
                antinodes.insert(nl1);
            }
            if nl2.0 >= 0 && nl2.0 < row_max && nl2.1 >= 0 && nl2.1 < col_max {
                antinodes.insert(nl2);
            }
        }
    }

    antinodes.len() as u32
}

pub fn solve_antinodes_two(
    antennes: HashMap<char, Vec<(i32, i32)>>,
    col_max: i32,
    row_max: i32,
) -> u32 {
    let mut antinodes = HashSet::new();
    for (_c, locations) in antennes.iter() {
        for l in locations.iter().combinations(2) {
            let l1 = *l[0];
            let l2 = *l[1];

            for an in calculate_all_antinodes(l1, l2, col_max, row_max) {
                antinodes.insert(an);
            }
        }
    }

    antinodes.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennes, (row_max, col_max)) = parse_input(input);
    //println!("dimensions: width={}, heigth={}", col_max, row_max);
    //println!("{:#?}", antennes);
    let result = solve_antinodes(antennes, col_max, row_max);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennes, (row_max, col_max)) = parse_input(input);
    let result = solve_antinodes_two(antennes, col_max, row_max);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
