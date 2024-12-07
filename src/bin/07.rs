advent_of_code::solution!(7);


use hashbrown::HashMap;
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

pub fn parse_input(input: &str) -> (Vec<Equation>, usize) {
    let mut max_op_space: usize = 0;
    let equations = input
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
            if op_space > max_op_space {
                max_op_space = op_space;
            }
            Equation {
                result,
                numbers,
                op_space,
            }
        })
        .collect();
    (equations, max_op_space)
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
    (left * next_power_of_10(right)) + right
    //let combined = format!("{}{}", left, right);
    //combined.parse::<u64>().expect("A valid combined number")
}

fn next_power_of_10(n: u64) -> u64 {
    if n == 0 {
        return 10;
    }
    let mut power = 1;
    while power <= n {
        power *= 10;
    }
    power
}

fn verify_operations(instructions: &Vec<Operators>, equation: &Equation) -> u64 {
    let mut ins_result = equation.numbers[0];
    let mut i = 1;
    for operator in instructions {
        ins_result = match operator {
            Operators::Add => ins_result + equation.numbers[i],
            Operators::Multiply => ins_result * equation.numbers[i],
            Operators::Combine => combine_number(ins_result, equation.numbers[i]),
        };
        // already too big so early exit
        if ins_result > equation.result {
            break;
        }

        i += 1
    }
    if ins_result == equation.result {
        ins_result
    } else {
        0
    }
}

fn valid_result(instructions: &Vec<Vec<Operators>>, equation: &Equation) -> u64 {
    let mut result: u64 = 0;
    for instruction in instructions {

        if verify_operations(instruction, equation) != 0 {
            result = equation.result;
            break;
        }
    }
    result
}

impl Equation {
    fn solve(&self, ops: &Vec<Vec<Operators>>) -> u64 {
        
        valid_result(ops, &self)
    }
}


// Creates map of all permutations for the Operators Enum
fn create_ops_permutations(max_ops_space: usize, operators: &[Operators]) -> HashMap<usize, Vec<Vec<Operators>>> {
    let mut ops_map: HashMap<usize, Vec<Vec<Operators>>> = HashMap::new();
    for i in 1..=max_ops_space {
        ops_map.insert(i, calculate_ops(operators, i));
    }
    ops_map
    
}

// Thanks to the example of https://github.com/nindalf/advent-2024/blob/master/src/day7/mod.rs#L43
// Improved execution time from 450ms to 60 ms
fn calculate_ops_early(operators: &[Operators], equation: &Equation) -> bool {
    let mut intermediates: Vec<u64> = vec![equation.numbers[0]];
    let N: usize = operators.len();
    for operand in equation.numbers.iter().skip(1) {
        let mut temp = Vec::with_capacity(intermediates.len() * N);
        for n in intermediates {
            let addition_result = n + operand;
            if addition_result <= equation.result {
                temp.push(addition_result);
            }
            let multiplication_result = n * operand;
            if multiplication_result <= equation.result {
                temp.push(multiplication_result);
            }
            if N == 3 {
                let concat_result = (n * next_power_of_10(*operand)) + *operand;
                if concat_result <= equation.result {
                    temp.push(concat_result);
                }
            }
        }
        intermediates = temp
    }

    intermediates.iter().any(|n| *n == equation.result)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (equations, max_op_size) = parse_input(input);
    //let ops_map = create_ops_permutations(max_op_size, &OPERATORS);
    //let result = equations.par_iter().map(|equation| equation.solve(ops_map.get(&equation.op_space).unwrap())).sum();
    let result = equations.iter().filter( |&e| {
        calculate_ops_early(&OPERATORS, e)
    }).map(|e| e.result).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (equations, max_op_size) = parse_input(input);
    //let ops_map = create_ops_permutations(max_op_size, &OPERATORS2);
    //let result = equations.par_iter().map(|equation| equation.solve(ops_map.get(&equation.op_space).unwrap())).sum();
    let result = equations.iter().filter( |&e| {
        calculate_ops_early(&OPERATORS2, e)
    }).map(|e| e.result).sum();
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
