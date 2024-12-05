advent_of_code::solution!(2);

use itertools::{self, Itertools};

pub fn calculate_safety(levels: &[i32]) -> bool {
    let level_pairs: Vec<(i32, i32)> = levels.iter().copied().tuple_windows().collect();
    level_pairs.iter().all(|(a, b)| (1..=3).contains(&(a - b)))
        || level_pairs.iter().all(|(a, b)| (1..=3).contains(&(b - a)))
}

pub fn dampen_levels(levels: &[Vec<i32>]) -> i32 {
    levels
        .iter()
        .filter(|&level| {
            // safe
            if calculate_safety(level) {
                true
            // unsafe, bepaal of de dampener effect heeft
            } else {
                (0..level.len())
                    .map(|j| {
                        level
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &val)| if j != i { Some(val) } else { None })
                            .collect::<Vec<i32>>()
                    })
                    .any(|dampened_level| calculate_safety(&dampened_level))
            }
        })
        .count() as i32
}

pub fn parse_level(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|c| c.parse::<i32>().expect("expected a number"))
        .collect()
}

pub fn parse_report(input: &str) -> i32 {
    let levels = parse_level(input);

    if calculate_safety(&levels) {
        1
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let result: i32 = input.lines().map(|l| parse_report(l)).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let levels: Vec<Vec<i32>> = input.lines().map(|l| parse_level(l)).collect();
    let result = dampen_levels(&levels);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
