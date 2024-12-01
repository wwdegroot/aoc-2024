advent_of_code::solution!(1);

use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

pub fn parse_numbers(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag("   "), i32)(input)
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_loc: Vec<i32> = Vec::with_capacity(1000);
    let mut second_loc: Vec<i32> = Vec::with_capacity(1000);
    for line in input.lines() {
        let (_, (first, second)) = parse_numbers(line).unwrap();
        first_loc.push(first);
        second_loc.push(second);
    }
    first_loc.sort();
    second_loc.sort();
    (first_loc, second_loc)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (first_loc, second_loc) = parse_input(input);
    let zipped: Vec<i32> = first_loc
        .into_iter()
        .zip(second_loc.iter())
        .map(|(a, b)| {
            let distance = a - b;
            distance.abs()
        })
        .collect();
    Some(zipped.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (first_loc, second_loc) = parse_input(input);
    // 1. tel hoevaak nummers voorkomen in second_loc, sla informatie op in hashmap
    // 2. loop door first_loc en vermenigvuldig dit met het gevonden aantal uit de hashmap
    // 3. let op dat key not found is 0
    // 4. sum het aantal
    let second_max = second_loc
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        });
    let result: Vec<i32> = first_loc
        .into_iter()
        .map(|f| {
            let val = second_max.get(&f);
            match val {
                Some(v) => f * v,
                None => 0_i32,
            }
        })
        .collect();
    Some(result.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
