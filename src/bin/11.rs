use std::iter::successors;
use hashbrown::HashMap;

advent_of_code::solution!(11);

pub fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| {
        line.split(' ').map(|n| n.parse::<u64>().expect("Expected a number"))
    }).flatten().collect()
}

pub fn is_even_digit(n: u64) -> bool {
    let n_length = successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count();
    if n_length % 2 == 0 {
        true
    } else {
        false
    }
}

pub fn split_number_in_halves(n: u64) -> Vec<u64> {
    // can we split the number from the byte slice?
    // let n_bytes: [u8; 8] = n.to_ne_bytes();
    // let middle = n_bytes.len() / 2;
    // let n_bytes_left: Vec<u8> = n_bytes.clone().into_iter().enumerate().filter(|(i, x)| i <= &middle).map(|(_, x)| x).collect();
    // let n_bytes_right: Vec<u8> = n_bytes.clone().into_iter().enumerate().filter(|(i, x)| i >= &middle).map(|(_, x)| x).collect();
    // println!("left={:?} right={:?}", n_bytes_left, n_bytes_right);
    // let left: u32 = u32::from_ne_bytes(n_bytes_left.as_slice().try_into().unwrap());
    // let right: u32 = u32::from_ne_bytes(n_bytes_right.as_slice().try_into().unwrap());
    // vec![u64::from(left),u64::from(right)]
    let number = n.to_string();
    let middle = number.len() / 2;
    let (left, right) = number.split_at(middle);
    vec![left.parse::<u64>().unwrap(),right.parse::<u64>().unwrap() ]
}

pub fn cycle_stones(mut stones: Vec<u64>, cycle_count: usize) -> usize {
    let mut iteration: usize = 0;
    while iteration < cycle_count {
        stones = stones.into_iter().map(|n| {
            let mut new_n: Vec<u64> = vec![];
            if n == 0 {
                new_n.push(1_u64);
            } else if is_even_digit(n) {
                new_n.extend(split_number_in_halves(n));
            } else {
                new_n.push(n*2024);
            }
            new_n
        }).flatten().collect();
        iteration += 1;
    }
    stones.len()
}

pub fn cycle_stones_cached(stones: Vec<u64>, cycle_count: usize) -> usize {
    let mut iteration: usize = 0;
    let mut map: HashMap<u64, usize> = HashMap::new();
    stones.iter().for_each(|n| *map.entry(*n).or_insert(0) +=1);
    // println!("{:#?}", map);
    while iteration < cycle_count {
        let mut cycle_map: HashMap<u64, usize> = HashMap::new();
        for (&k, &n) in map.iter() {
            if k == 0 {
                *cycle_map.entry(1).or_insert(0) += n;

            } else if is_even_digit(k){
                let new_numbers = split_number_in_halves(k);
                for nn in new_numbers {
                    *cycle_map.entry(nn).or_insert(0) += n;
                }

            } else {
                let new_number = k *2024;
                *cycle_map.entry(new_number).or_insert(0) += n;
            }
        }
        map = cycle_map;
        iteration += 1;
    }
    // println!("{:#?}", map);
    map.values().sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones = parse_input(input);
    let cycle_count = 25;
    let result = cycle_stones_cached(stones, cycle_count);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = parse_input(input);
    let cycle_count = 75;
    let result = cycle_stones_cached(stones, cycle_count);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_split_number_in_halves(){
        let result = split_number_in_halves(4010);
        let expected: Vec<u64> = vec![40, 10];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
