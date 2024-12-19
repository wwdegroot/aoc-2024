use itertools::Itertools;
use hashbrown::HashMap;

advent_of_code::solution!(19);


pub fn count_valid_patterns(
    input: &str,
    patterns: &Vec<String>,
    cached: &mut HashMap<String, usize>,
    single: bool,
) -> usize {
    if input.is_empty() {
        return 1;
    }

    if let Some(&result) = cached.get(input) {
        return result;
    }

    let mut count = 0;

    for p in patterns.into_iter() {
        if input.starts_with(p) {
            let suffix = &input[p.len()..];
            let result = count_valid_patterns(suffix, patterns, cached, single);
            if single && result == 1 {
                cached.insert(input.to_string(), 1);
                return 1;
            }
            count += result;
        }
    }

    cached.insert(input.to_string(), count);
    count
}

pub fn parse_input(input: &str) -> (Vec<&str>, Vec<String>) {
    let pattern_str = input.lines().collect_vec()[0];
    let colorcodes: Vec<String> = pattern_str.split(", ").map(|code| code.to_owned() ).collect();

    let inputs = input.lines()
    .enumerate()
    .filter_map(|(i, l)| {if i > 1 { Some(l)} else { None} })
    .collect_vec();
    (inputs, colorcodes)
}


pub fn part_one(input: &str) -> Option<usize> {
    let (inputs, patterns) = parse_input(input);
    let mut cached: HashMap<String, usize> = HashMap::new();
    let result = inputs.iter().map(|input| count_valid_patterns(input, &patterns, &mut cached, true)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (inputs, patterns) = parse_input(input);
    let mut cached: HashMap<String, usize> = HashMap::new();
    let result = inputs.iter().map(|input| count_valid_patterns(input, &patterns, &mut cached, false)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
