advent_of_code::solution!(22);

use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;
use itertools::Itertools;


const MULT: isize = 64;
const MODULO: isize = 16777216;
const DIVI: isize = 32;
const MULT2: isize = 2048;


pub fn evolve_secret(number: &isize, iterations: usize) -> isize {
    let mut secret: isize = *number;
    for _i in 0..iterations {
        // step 1
        // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ (secret * MULT)).rem_euclid(MODULO);
        // step 2
        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ secret.div_euclid(DIVI)).rem_euclid(MODULO);
        // step 3
        // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ (secret * MULT2)).rem_euclid(MODULO);

    }
    secret
}


pub fn evolve_secret_bananas(number: &isize, iterations: usize) -> Vec<isize> {
    let mut bananas: Vec<isize> = Vec::with_capacity(iterations);
    let mut secret: isize = *number;

    for _ in 0..iterations {
        // step 1
        // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ (secret * MULT)).rem_euclid(MODULO);
        // step 2
        // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ secret.div_euclid(DIVI)).rem_euclid(MODULO);
        // step 3
        // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
        secret = (secret ^ (secret * MULT2)).rem_euclid(MODULO);
        bananas.push(secret % 10);
    }
    bananas
}


pub fn parse_input(input: &str) -> Vec<isize> {
    input.lines().map(|n| n.parse::<isize>().expect("Expected a number here")).collect()
}


pub fn part_one(input: &str) -> Option<isize> {
    let seeds = parse_input(input);
    let iterations: usize = 2000;
    let result = seeds.par_iter().map(|s| evolve_secret(s, iterations)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let seeds = parse_input(input);
    let iterations: usize = 2000;
    let mut counts: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
    let mbananas: Vec<Vec<isize>> = seeds.iter().map(|s| evolve_secret_bananas(s, iterations)).collect();
    for b in mbananas.into_iter() {
        let mut set = HashSet::new();
        b.windows(5).for_each(|b| {
            let diffs = (b[1] - b[0], b[2] - b[1], b[3] - b[2], b[4] - b[3]);
            let amount = b[4];
            if set.insert(diffs) {
                counts.entry(diffs).and_modify(|e| *e += amount as usize).or_insert(amount as usize);
            }

        });
    }
    let result = counts.values().max().unwrap();
    Some(*result)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let answer = 42 ^ 15;
        assert_eq!(answer, 37);
    }

    #[test]
    fn test_prune() {
        let answer = 100_000_000_isize.rem_euclid(MODULO);
        assert_eq!(answer, 16113920);
    }

    #[test]
    fn test_evolving_one() {
        let answer = evolve_secret(&123, 1);
        assert_eq!(answer, 15887950);
    }

    #[test]
    fn test_evolving_ten() {
        let answer = evolve_secret(&10, 2000);
        assert_eq!(answer, 4700978);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
