advent_of_code::solution!(23);

use hashbrown::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<(String, String)> {
    input.lines().map(|l| {
        if let Some((left,  right)) = l.split_once("-") {
            (left.to_string(), right.to_string())
        } else {
            panic!("Unexpected input");
        }
    }).collect()
}

pub fn new_hashset(value: String) -> HashSet<String> {
    let mut h = HashSet::new();
    h.insert(value);
    h
}

pub fn part_one(input: &str) -> Option<usize> {
    let edges= parse_input(input);
    // let mut computers: HashSet<String> = HashSet::with_capacity(2000);
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (left, right) in edges {
        // computers.insert(left.clone());
        // computers.insert(right.clone());
        connections.entry(left.clone()).and_modify(|v| {v.insert(right.clone());}).or_insert(new_hashset(right.clone()));
        connections.entry(right.clone()).and_modify(|v| {v.insert(left.clone());}).or_insert(new_hashset(left.clone()));
    }
    // println!("{:#?}", connections);
    let mut three: HashSet<(String, String, String)> = HashSet::new();
    for (pc1, conn1) in connections.iter() {
        if pc1.starts_with("t") {
            for (pc2, conn2 ) in connections.iter() {
                for (pc3, conn3) in connections.iter() {
                    if conn1.contains(pc2) && conn1.contains(pc3) && conn2.contains(pc1) && conn2.contains(pc3) && conn3.contains(pc1) && conn3.contains(pc2){
                        // three connected pc's
                        let mut hit: Vec<String> = vec![pc1.clone(), pc2.clone(), pc3.clone()];
                        hit.sort_unstable();
                        three.insert((hit[0].clone(), hit[1].clone(), hit[2].clone()));
                    }
                }
    
            }
        }
    }
    // println!("{:#?}", three);
    let result = three.len();
    Some(result)

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
