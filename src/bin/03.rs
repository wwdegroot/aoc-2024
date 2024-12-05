advent_of_code::solution!(3);

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::i32;
use nom::combinator::{iterator, opt};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

use regex::Regex;

fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(","), i32)(input)
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    // pakt nog teveel, closing tag moet na een nummer zijn, nu kan 'mul(32,64] then(mul(11,8)' niet correct geparsed worden
    delimited(tag("mul("), parse_integer_pair, tag(")"))(input)
}

fn parse_input(input: &str) -> IResult<&str, (i32, i32)> {
    let (remaining, _taken) = take_until("mul(")(input)?;
    parse_mul(remaining)
}

pub fn parse_part_one(input: &str) -> i32 {
    // fails on error, so not enough results
    let mut nom_it = iterator(input, parse_input);

    let result: Vec<i32> = nom_it
        .map(|(a, b)| {
            println!("{:?}", (a, b));
            a * b
        })
        .collect();

    let parser_result: IResult<_, _> = nom_it.finish();
    let (remaining_input, ()) = parser_result.unwrap();
    println!("remaining={:?}", remaining_input);
    result.iter().sum()
}

fn regex_part_one(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3})+,([0-9]{1,3})+\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum()
}

fn regex_part_two(input: &str) -> i32 {
    let mut enabled = true;
    let mut result = 0;
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let ge = Regex::new(r"\d{1,3}").unwrap();
    for pattern in re.find_iter(&input) {
        let part = pattern.as_str();
        match part {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if !enabled {
                    continue;
                }
                let mut numbers = ge.find_iter(part);
                let a: i32 = numbers.next().unwrap().as_str().parse().unwrap();
                let b: i32 = numbers.next().unwrap().as_str().parse().unwrap();
                result += a * b;
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<i32> {
    // let nom_res = parse_part_one(input);
    // println!("nom={}", nom_res);
    Some(regex_part_one(input))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(regex_part_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)";
        let result = part_two(test_input);
        assert_eq!(result, Some(48));
    }
}
