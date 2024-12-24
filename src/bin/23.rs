advent_of_code::solution!(23);

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

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
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (left, right) in edges {
        connections.entry(left.clone()).and_modify(|v| {v.insert(right.clone());}).or_insert(new_hashset(right.clone()));
        connections.entry(right.clone()).and_modify(|v| {v.insert(left.clone());}).or_insert(new_hashset(left.clone()));
    }
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
    let result = three.len();
    Some(result)

}

// Algorithm to find max size sub graph in nodes https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bronkerbosch(connections: &HashMap<String, HashSet<String>>, r: HashSet<String>, mut p: HashSet<String>, mut x: HashSet<String>, cliques: &mut Vec<HashSet<String>>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else if !p.is_empty() {
        let nodes = p.iter().cloned().collect::<HashSet<String>>();
        nodes.iter().for_each(|node| {
            let neighbours: &HashSet<String> = connections.get(node).unwrap();
            let mut to_add: HashSet<String> = HashSet::<String>::new();
            to_add.insert(node.clone());
            bronkerbosch(
                connections,
                r.union(&to_add).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                cliques,
            );
            p.remove(node);
            x.insert(node.clone());
        });
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let edges= parse_input(input);
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (left, right) in edges {
        connections.entry(left.clone()).and_modify(|v| {v.insert(right.clone());}).or_insert(new_hashset(right.clone()));
        connections.entry(right.clone()).and_modify(|v| {v.insert(left.clone());}).or_insert(new_hashset(left.clone()));
    }
    let r: Vec<String> = connections.keys().cloned().collect();
    let mut cliques: Vec<HashSet<String>> = Vec::new();
    bronkerbosch(&connections, HashSet::new(), HashSet::from_iter(r), HashSet::new(), &mut cliques);
    cliques.sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
    let password = find_max_cliques(&cliques);
    Some(password)
}

fn find_max_cliques(cliques: &[HashSet<String>]) -> String {
    let largest = cliques.first().cloned().unwrap();
    let mut largest_sorted: Vec<String> = largest.into_iter().collect();
    largest_sorted.sort_unstable();
    let password = largest_sorted.iter().join(",");
    password
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
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
