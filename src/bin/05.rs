advent_of_code::solution!(5);

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct PageOrderRule {
    first: i32,
    last: i32,
}

#[derive(Debug, Clone)]
pub struct PageUpdate {
    numbers: Vec<i32>,
}

pub fn parse_input(input: &str) -> (Vec<PageOrderRule>, Vec<PageUpdate>) {
    let mut rules: Vec<PageOrderRule> = Vec::with_capacity(1200);
    let mut pages: Vec<PageUpdate> = Vec::with_capacity(300);
    let mut rules_active: bool = true;
    for line in input.lines() {
        if line.is_empty() {
            rules_active = false;
            continue;
        }
        if rules_active {
            let (a, b) = line.split_once("|").expect("Expected page rule");
            let first = a.parse::<i32>().expect("Not a number");
            let last = b.parse::<i32>().expect("Not a number");
            rules.push(PageOrderRule { first, last });
        } else {
            let numbers: Vec<i32> = line
                .split(",")
                .map(|n| n.parse::<i32>().expect("Expected a number here"))
                .collect();
            pages.push(PageUpdate { numbers });
        }
    }
    (rules, pages)
}

pub fn check_pages(pages: Vec<PageUpdate>, rules: &Vec<PageOrderRule>) -> i32 {
    pages
        .iter()
        .map(|p| {
            let mut correct: bool = true;
            for r in rules {
                let first_pos = p.numbers.iter().position(|&x| x == r.first);
                let last_pos = p.numbers.iter().position(|&x| x == r.last);
                // rule numbers or number not found in this page, contine with other rules
                if first_pos.is_none() || last_pos.is_none() {
                    continue;
                }
                // page failed the rule check, check next page
                if first_pos.unwrap() > last_pos.unwrap() {
                    correct = false;
                    break;
                }
            }
            // return middle number of page numbers
            if correct {
                let middle_index: usize = p.numbers.len() / 2;
                p.numbers[middle_index]
            } else {
                0
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    let (rules, pages) = parse_input(input);
    let result = check_pages(pages, &rules);
    Some(result)
}

pub fn sort_numbers(mut numbers: Vec<i32>, rules: &Vec<PageOrderRule>) -> (bool, Vec<i32>) {
    let mut sorted: bool = false;
    for r in rules {
        let first_pos = numbers.iter().position(|&x| x == r.first);
        let last_pos = numbers.iter().position(|&x| x == r.last);
        // rule numbers not found in this page, contine with other rules
        if first_pos.is_none() || last_pos.is_none() {
            continue;
        }
        // page failed the rule insert the index of the first before the last and continue
        if first_pos.unwrap() > last_pos.unwrap() {
            // now insert first on the index of last and last on the index of first
            numbers.remove(last_pos.unwrap());
            numbers.insert(last_pos.unwrap(), r.first);
            numbers.remove(first_pos.unwrap());
            numbers.insert(first_pos.unwrap(), r.last);
            sorted = true;
            break;
        }
    }
    (sorted, numbers)
}

pub fn fix_pages(pages: Vec<PageUpdate>, rules: &Vec<PageOrderRule>) -> Vec<Option<i32>> {
    pages
        .into_iter()
        .map(|mut p| {
            let mut sorted = false;
            let mut numbers: Vec<i32> = p.numbers.clone();
            let mut middle: Option<i32> = None;
            // keep looping till no rules cause a sort
            let mut i: usize = 1;
            loop {
                (sorted, numbers) = sort_numbers(numbers, rules);
                if !sorted {
                    // needed at least one sorting, so was a failing pageupdate
                    if i > 1 {
                        middle = Some(numbers[numbers.len() / 2]);
                    }
                    break;
                }
                i += 1
            }
            middle
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<i32> {
    let (rules, pages) = parse_input(input);
    let fixed_pages: Vec<Option<i32>> = fix_pages(pages, &rules);
    let result = fixed_pages.iter().filter_map(|n| *n).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
