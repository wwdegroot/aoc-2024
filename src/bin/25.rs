advent_of_code::solution!(25);

#[derive(Debug, Clone)]
pub enum LockOrKey {
    Key{ data: Vec<isize>},
    Lock{ data: Vec<isize>},
}

impl LockOrKey {

    fn is_lock(&self) -> bool {
        match self {
            LockOrKey::Lock{data: _}=> {
                true
            },
            LockOrKey::Key{data: _} => {
                false
            }
        }
    }

    fn is_key(&self) -> bool {
        match self {
            LockOrKey::Lock{data: _}=> {
                false
            },
            LockOrKey::Key{data: _} => {
                true
            }
        }
    }

    fn data(&self) -> &Vec<isize> {
        match self {
            LockOrKey::Key { data, } => data,
            LockOrKey::Lock { data, } => data,
        }
    }

    fn fits(&self, other: &LockOrKey) -> bool {
        let mut fitting: bool = true;
        let data = self.data();
        let other_data = other.data();
        // println!("key={:?}, lock={:?}", data, other_data);
        for (n1, n2) in data.iter().zip(other_data.iter()) {
            if n1 + n2 > 5 {
                fitting = false;
                break;
            }
        }
        fitting
    }

}

pub fn parse_lock_or_key(input: &str) -> LockOrKey {
    let (mut col1, mut col2, mut col3, mut col4, mut col5) = (0_isize, 0_isize, 0_isize, 0_isize ,0_isize);
    let lock: bool;
    let mut row: usize = 0;
    let lines: Vec<&str> = input.lines().collect();
    // decide lock or key
    match lines[row] {
        "#####" => {
            lock = true;
        },
        "....." => {
            lock = false;
        },
        _ => !unreachable!(),
    }
    row +=1;
    while row < lines.len() {
        for (i, l) in lines[row].chars().enumerate() {
            if i == 0 && l == '#' {
                col1 += 1;
            } else if i == 1 && l == '#'  {
                col2 += 1;
            } else if i == 2 && l == '#'  {
                col3 += 1;
            } else if i == 3 && l == '#'  {
                col4 += 1;
            } else if i == 4 && l == '#'  {
                col5 += 1;
            } 
        }
        row +=1;
    }
    
    if lock {
        let data = vec![col1, col2, col3, col4, col5];
        return LockOrKey::Lock{data}
    } else {
        let data = vec![col1 -1, col2 -1, col3 -1, col4 -1, col5 -1];
        return LockOrKey::Key{data}
    }
}


pub fn parse_input(input: &str) -> Vec<LockOrKey> {
    let data: Vec<&str> = input.split("\n\n").collect();
    data.into_iter().map(|d| parse_lock_or_key(d)).collect()

}

pub fn part_one(input: &str) -> Option<u32> {
    let schematics = parse_input(input);
    let keys: Vec<LockOrKey> = schematics.clone().into_iter().filter(|s| s.is_key()).collect();
    let locks: Vec<LockOrKey> = schematics.clone().into_iter().filter(|s| s.is_lock()).collect();

    let mut key_fit_count: u32 = 0;
    //match the keys and count how many keys fit inside a lock
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.fits(lock) {
                key_fit_count += 1
            }
        }
    }
    Some(key_fit_count)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
