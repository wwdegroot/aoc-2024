advent_of_code::solution!(7);

use std::fmt::format;

use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Operators {
    Add,
    Multiply,
    Combine,
}

pub const OPERATORS: [Operators; 2] = [Operators::Add, Operators::Multiply];
pub const OPERATORS2: [Operators; 3] = [Operators::Add, Operators::Multiply, Operators::Combine];

pub struct Equation {
    result: u64,
    numbers: Vec<u64>,
    op_space: usize,
}

fn calculate_ops(operators: &[Operators], length: usize) -> Vec<Vec<Operators>> {
    if length == 1 {
        operators.iter().map(|&o| vec![o]).collect()
    } else {
        let mut permutations = Vec::new();
        for op in operators {
            for perm in calculate_ops(operators, length - 1) {
                let mut new_perm = vec![*op];
                new_perm.extend(perm);
                permutations.push(new_perm);
            }
        }
        permutations
    }
}

fn combine_number(left: u64, right: u64) -> u64 {
    let combined = format!("{}{}", left, right);
    combined.parse::<u64>().expect("A valid combined number")
}

fn valid_result(instructions: Vec<Vec<Operators>>, equation: &Equation) -> u64 {
    let mut valid_count = 0;
    for instruction in instructions {
        let mut ins_result = equation.numbers[0];
        let mut i = 1;
        for operator in instruction {
            ins_result = match operator {
                Operators::Add => ins_result + equation.numbers[i],
                Operators::Multiply => ins_result * equation.numbers[i],
                Operators::Combine => combine_number(ins_result, equation.numbers[i]),
            };
            i += 1
        }
        if ins_result == equation.result {
            //println!("valid ={:?}", ins_result);
            valid_count += 1;
            break;
        }
    }
    if valid_count > 0 {
        equation.result
    } else {
        0
    }
}

impl Equation {
    fn solve(&self) -> u64 {
        let ops = calculate_ops(&OPERATORS, self.op_space);

        //println!("numbers= {:?} has {:?}", self.numbers, ops);
        valid_result(ops, &self)
    }

    fn solve2(&self) -> u64 {
        let ops = calculate_ops(&OPERATORS2, self.op_space);

        //println!("numbers= {:?} has {:?}", self.numbers, ops);
        valid_result(ops, &self)
    }
}

pub fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (resultstr, numberstr) = l.split_once(": ").expect("Expected an eqution line");
            let numbers: Vec<u64> = numberstr
                .split(" ")
                .map(|n| n.parse::<u64>().expect("A number that could be parsed"))
                .collect();
            let result = resultstr
                .parse::<u64>()
                .expect("A number before the : to parse.");
            let op_space = numbers.len() - 1;
            Equation {
                result,
                numbers,
                op_space,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let result = equations.par_iter().map(|equation| equation.solve()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let result = equations.par_iter().map(|equation| equation.solve2()).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
