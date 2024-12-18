use std::collections::HashMap;
use std::ops::Div;

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Computer {
    registers: HashMap<String, i64>,
    operations: Vec<i64>,
    outputs: Vec<i64>,
    pointer: usize,
}

fn process_operation(computer: &mut Computer, opcode_pos: usize) -> Option<usize> {
    let (opcode, literal_val) = (
        computer.operations[opcode_pos],
        computer.operations[opcode_pos + 1]
    );

    let mut combo_val: i64 = 0;
    if literal_val <= 3 {
        // Litteral value
        combo_val = literal_val;
    } else if literal_val == 4 {
        // Register A
        combo_val = *computer.registers.get("A").unwrap();
    } else if literal_val == 5 {
        // Register B
        combo_val = *computer.registers.get("B").unwrap();
    } else if literal_val == 6 {
        // Register C
        combo_val = *computer.registers.get("C").unwrap();
    } else {
        // Nothing **FOR NOW**
    }

    let (a, b, c) = (
        *computer.registers.get("A").unwrap(),
        *computer.registers.get("B").unwrap(),
        *computer.registers.get("C").unwrap()
    );

    match opcode {
        0 => {
            // println!("Ops<0> - A = a / {}", 2_i32.pow(combo_val as u32));
            computer.registers.insert("A".to_string(), a / 2_i32.pow(combo_val as u32) as i64);
        },
        1 => {
            // println!("Ops<1> - B = b ^ {}", literal_val);
            computer.registers.insert("B".to_string(), b ^ literal_val);
        },
        2 => {
            // println!("Ops<2> - B = {} % 8", combo_val);
            computer.registers.insert("B".to_string(), combo_val % 8);
        },
        3 => {
            if a > 0 {
                // println!("Ops<3> - jump({})", literal_val);
                return Some(literal_val as usize);
            }
        },
        4 => {
            // println!("Ops<4> - B = b ^ c");
            computer.registers.insert("B".to_string(), b ^ c);
        },
        5 => {
            // println!("Ops<5> - OUT({} % 8)", combo_val);
            computer.outputs.push(combo_val % 8)
        },
        6 => {
            // println!("Ops<6> - B = a / {}", 2_i32.pow(combo_val as u32));
            computer.registers.insert("B".to_string(), a / 2_i32.pow(combo_val as u32) as i64);
        },
        7 => {
            // println!("Ops<7> - C = a / {}", 2_i32.pow(combo_val as u32));
            computer.registers.insert("C".to_string(), a / 2_i32.pow(combo_val as u32) as i64);
        },
        _ => unimplemented!()
    }

    None
}

fn parse_input(input: &str) -> Computer {
    let mut registers = HashMap::new();
    let mut operations = Vec::new();

    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse registers
    for line in parts[0].lines() {
        if let Some((key, value)) = line.split_once(": ") {
            registers.insert(key.trim().replace("Register ", "").to_string(), value.trim().parse().unwrap());
        }
    }

    // Parse program
    if let Some(prog_str) = parts.get(1) {
        let numbers: Vec<i64> = prog_str.trim().split(&[',', ' '][..])
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        operations.extend(numbers);
    }

    Computer { registers, operations, outputs: Vec::new(), pointer: 0 }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = parse_input(input);

    let mut current_pointer: usize = 0;
    loop {
        if computer.operations.len() <= current_pointer {
            break;
        }

        let jump = process_operation(&mut computer, current_pointer);
        if jump.is_some() {
            current_pointer = jump.unwrap();
        } else {
            current_pointer += 2;
        }
        println!("{:?}", computer);
    }

    Some(computer.outputs.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
}

struct Computer2<'a> {
    program: &'a [u64],
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl Computer2<'_> {
    fn run(&mut self) -> Option<u64> {
        while self.ip < self.program.len() {
            let combo = |index| match self.program[index] {
                0..4 => self.program[index],
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            };

            match self.program[self.ip] {
                0 => self.a >>= combo(self.ip + 1),
                1 => self.b ^= self.program[self.ip + 1],
                2 => self.b = combo(self.ip + 1) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = self.program[self.ip + 1] as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    let out = combo(self.ip + 1) % 8;
                    self.ip += 2;
                    return Some(out);
                }
                6 => self.b = self.a >> combo(self.ip + 1),
                7 => self.c = self.a >> combo(self.ip + 1),
                _ => unreachable!(),
            }

            self.ip += 2;
        }

        None
    }
}


pub fn part_two(input: &str) -> Option<u64> {
    let cmpt = parse_input(input);
    let input_numbers: Vec<u64> = cmpt.operations.iter().map(|o| *o as u64).collect();
    let operations = input_numbers.clone();

    let mut valid = vec![0];

    for &out in input_numbers.iter().rev() {
        let mut next = Vec::new();

        for v in valid {
            for n in 0..8 {
                let a = (v << 3) | n;
                let mut computer =
                    Computer2 {
                        program: &operations, ip: 0,
                        a,
                        b: *cmpt.registers.get("B").unwrap() as u64,
                        c: *cmpt.registers.get("C").unwrap() as u64,
                    };

                if let Some(result) = computer.run() {
                    if result == out {
                        next.push(a);
                    }
                }
            }
        }

        valid = next;
    }

    Some(*valid.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
