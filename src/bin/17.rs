use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug)]
pub struct Computer {
    A: i64,
    B: i64,
    C: i64,
    program: Vec<i64>,
    ipointer: usize,
    output: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instructions {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}




impl Computer {

    fn new() -> Computer {
        Computer { A: 0, B: 0, C: 0, program: vec![], ipointer: 0, output: vec![] }
    }

    fn execute(&mut self) {
        let mut early_exit_at: i64 = 0;
        while self.ipointer < self.program.len() {
            if early_exit_at > 10_000 {
                break;
            }
            let opcode = self.program[self.ipointer];
            let operand = self.program[self.ipointer + 1];
            self.ipointer += 2;
            // println!("opcode={opcode}, operand={operand}");
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => !unreachable!(),
            };
            // println!("output={:?}", self.output);
            early_exit_at += 1;
        }

    }
    // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. 
    // The denominator is found by raising 2 to the power of the instruction's combo operand. 
    // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) 
    // The result of the division operation is truncated to an integer and then written to the A register.
    fn adv(&mut self, operand: i64) {
        self.A = self.A / ( 1 << self.get_operand(operand));
    }
    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, 
    // then stores the result in register B.
    fn bxl(&mut self, operand: i64) {
        self.B = self.B ^ operand;
    }

    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (((a % b) + b) % b) (thereby keeping only its lowest 3 bits), 
    // then writes that value to the B register.
    fn bst(&mut self, operand: i64) {
        self.B = self.get_operand(operand) % 8;
    }
    // The jnz instruction (opcode 3) does nothing if the A register is 0. 
    // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; 
    // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    fn jnz(&mut self, operand: i64) {
        if self.A != 0 {
            self.ipointer = operand as usize;
        } else {
            self.ipointer +=2;
        }
    }
    
    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. 
    // (For legacy reasons, this instruction reads an operand but ignores it.)
    fn bxc(&mut self, _operand: i64) {
        self.B = self.B ^ self.C;
    }

    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. 
    // (If a program outputs multiple values, they are separated by commas.)
    fn out(&mut self, operand: i64) {
        self.output.push(self.get_operand(operand) % 8);
    }

    // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    fn bdv(&mut self, operand: i64) {
        self.B = self.A / ( 1 << self.get_operand(operand));
    }
    // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
    fn cdv(&mut self, operand: i64) {
        self.C = self.A / ( 1 << self.get_operand(operand));
    }
    
    fn get_operand(&self, n: i64) -> i64 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            7 => 7,
            _ => !unreachable!(),
        }
    }

    fn print(&self) -> String {
        self.output.iter().join(",")
    }

    fn from_input(input: &str) -> Computer {
        let mut A: i64 = 0;
        let mut B: i64 = 0;
        let mut C: i64 = 0;
        let mut program: Vec<i64> = vec![];
        for (i, line) in input.lines().enumerate() {
            match i {
                0 => {
                    //register A
                    let (_, register_a) = line.split_once(": ").unwrap();
                    A = register_a.parse::<i64>().expect("A Register number");
                },
                1 => {
                    // register B
                    let (_, register_b) = line.split_once(": ").unwrap();
                    B = register_b.parse::<i64>().expect("B Register number");
                },
                2 => {
                    // register C
                    let (_, register_c) = line.split_once(": ").unwrap();
                    C = register_c.parse::<i64>().expect("C Register number");
                },
                3 => {
                    // nothing
                },
                4 => {
                    // program
                    let (_, program_str) = line.split_once(": ").unwrap();
                    program = program_str.chars().filter(|c| *c != ',' ).map(|c| c.to_string().parse::<i64>().expect("A number here")).collect()
                },
                _ => !unreachable!()
            }
        }
        Computer { A, B, C, program, ipointer: 0, output: vec![] }
    }

}


pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::from_input(input);
    // println!("{:#?}",computer);
    computer.execute();
    let output = computer.print();
    Some(output)
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
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    
    #[test]
    fn test_computer() {
        let mut computer = Computer::new();
        computer.C = 9;
        computer.program = vec![2, 6];
        computer.execute();
        assert!(computer.B == 1)
    }

    #[test]
    fn test_computer2() {
        let mut computer = Computer::new();
        computer.A = 10;
        computer.program = vec![5,0,5,1,5,4];
        computer.execute();

        assert!(computer.print() == "0,1,2".to_string())
    }

    #[test]
    fn test_computer3() {
        let mut computer = Computer::new();
        computer.A = 2024;
        computer.program = vec![0,1,5,4,3,0];
        computer.execute();
        assert!(computer.A == 0);
        assert!(computer.print() == "4,2,5,6,7,7,7,7,3,1,0".to_string())
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
