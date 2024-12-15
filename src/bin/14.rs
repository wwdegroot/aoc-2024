advent_of_code::solution!(14);

const WIDTH: i32 = 101;
const HEIGTH: i32 = 103;

pub struct Coordinate {
    x: i32,
    y: i32,
}

pub struct Instruction {
    start: Coordinate,
    velocity: Coordinate,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line|{
        let (first, second) = line.split_once(" ").unwrap();
        let (_, start_str) = first.split_once('=').unwrap();
        let (_, vel_str) = second.split_once('=').unwrap();
        let (x, y) = start_str.split_once(",").unwrap();
        let (dx, dy) = vel_str.split_once(",").unwrap();
        Instruction{
            start: Coordinate { x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap() },
            velocity: Coordinate { x: dx.parse::<i32>().unwrap(), y: dy.parse::<i32>().unwrap() }
        }
    }).collect()

}

pub fn calculate_final_positions(
    instructions: Vec<Instruction>,
    width: i32,
    heigth: i32,
    iterations: i32
) -> Vec<Coordinate> {
    instructions.iter().map(|ins| {
        Coordinate{
            x: (ins.start.x + (ins.velocity.x * iterations)).rem_euclid(width),
            y: (ins.start.y + (ins.velocity.y * iterations)).rem_euclid(heigth),
        }
    }).collect()
}

pub fn score_quadrants(width: i32, heigth: i32, positions: Vec<Coordinate>) -> u32 {
    let middle_x = width.div_euclid(2);
    let middle_y: i32 = heigth.div_euclid(2);
    let (mut tr, mut tl, mut br, mut bl) = (0, 0, 0, 0);
    for p in positions {
        if p.x < middle_x {
            if p.y < middle_y {
                tl +=1;
            }
            else if p.y > middle_y {
                bl +=1;
            }
        }
        if p.x > middle_x {
            if p.y < middle_y {
                tr +=1;
            }
            else if p.y > middle_y {
                br +=1;
            }
        }
    }
    tr * tl * br * bl
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let positions = calculate_final_positions(instructions, WIDTH, HEIGTH, 100);
    let result = score_quadrants(WIDTH, HEIGTH, positions);
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
        assert_eq!(result, Some(222208000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
