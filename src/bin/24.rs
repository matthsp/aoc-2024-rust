use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Operand {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Input {
    id: String,
    val: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Operation {
    i1: String,
    i2: String,
    o: String,
    operand: Operand,
}

impl Operation {
    fn is_first(&self) -> bool {
        self.i1.eq("x00") || self.i2.eq("x00")
    }

    fn is_direct(&self) -> bool {
        self.i1.starts_with("x") || self.i2.starts_with("x")
    }

    fn is_output(&self) -> bool {
        self.o.starts_with("z")
    }

    fn has_operand(&self, operand: Operand) -> bool {
        self.operand == operand
    }

    fn has_input(&self, input: String) -> bool {
        self.i1 == input || self.i2 == input
    }

    fn has_output(&self, output: String) -> bool {
        self.o == output
    }
}

fn parse(input: &str) -> (HashMap<String, u8>, Vec<Operation>) {
    let (inputs, operations) = input.split_once("\n\n").expect("Invalid input format");

    let result_inputs: HashMap<String, u8> = inputs
        .lines()
        .map(|l| {
            let (id, val) = l.split_once(": ").expect("Invalid input format for inputs");
            (
                id.to_string(),
                match val {
                    "0" => 0,
                    "1" => 1,
                    _ => unimplemented!("Only '0' and '1' are supported as values"),
                },
            )
        })
        .collect();

    let result_operations: Vec<Operation> = operations
        .lines()
        .map(|l| {
            // Example: x27 XOR y27 -> vpt
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() != 5 {
                panic!("Invalid input format for operations");
            }
            Operation {
                i1: parts[0].to_string(),
                i2: parts[2].to_string(),
                o: parts[4].to_string(),
                operand: match parts[1] {
                    "AND" => Operand::AND,
                    "OR" => Operand::OR,
                    "XOR" => Operand::XOR,
                    _ => unimplemented!("Unsupported operand"),
                },
            }
        })
        .collect();

    (result_inputs, result_operations)
}

fn process_operations(inputs: &mut HashMap<String, u8>, operations: &Vec<Operation>) -> Vec<Input> {
    // Separate operations doable from the start and the calculated ones
    let mut starts_with_xy: Vec<Operation> = Vec::new();
    let mut others: Vec<Operation> = Vec::new();

    operations.iter().for_each(|op| {
        if op.i1.starts_with('x') || op.i1.starts_with('y') {
            starts_with_xy.push(op.clone());
        } else {
            others.push(op.clone());
        }
    });

    starts_with_xy.iter().for_each(|o| {
        let i1 = inputs.get(&o.i1).expect("Input not found");
        let i2 = inputs.get(&o.i2).expect("Input not found");
        let op_result: u8 = match o.operand {
            Operand::AND => i1 & i2,
            Operand::OR => i1 | i2,
            Operand::XOR => i1 ^ i2,
        };
        inputs.insert(o.o.clone(), op_result);
    });

    others.sort_by(|a, b| a.o.cmp(&b.o));

    let mut to_process: Vec<Operation> = others.clone();
    loop {
        let mut leftovers: Vec<Operation> = Vec::new();

        // Sort others alphabetically by output
        to_process.iter().for_each(|o| {
            if inputs.get(&o.i1).is_none() || inputs.get(&o.i2).is_none() {
                leftovers.push(o.clone());
                return;
            }
            let i1 = inputs.get(&o.i1).unwrap();
            let i2 = inputs.get(&o.i2).unwrap();
            let op_result: u8 = match o.operand {
                Operand::AND => i1 & i2,
                Operand::OR => i1 | i2,
                Operand::XOR => i1 ^ i2,
            };
            inputs.insert(o.o.clone(), op_result);
        });

        if leftovers.len() > 0 {
            to_process = leftovers;
        } else {
            break;
        }
    }

    inputs.iter()
        .map(|(k, &val)| Input { id: k.clone(), val })
        .collect()
}

fn vec_to_decimal(bits: Vec<Input>) -> u64 {
    bits.iter() // Reverse the vector so the most significant bit is first
        .enumerate()  // Get both index and value
        .fold(0, |acc, (i, bit)| acc + (bit.val as u64) * (2u64.pow(i as u32))) // Sum the bits to the result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut inputs, operations) = parse(input);

    // Process data
    let mut bits: Vec<Input> = process_operations(&mut inputs, &operations);

    bits.retain(|i| i.id.starts_with("z"));
    bits.sort_by(|a, b| a.id.cmp(&b.id));

    let value = vec_to_decimal(bits);
    Some(value)
}

// Find suspicious XOR operation related to xN and yN
// Except for x00/y00, none should have a zN output
fn find_suspicious_direct_xor_operations(operations: &Vec<Operation>) -> Vec<String> {
    let mut sus = Vec::new();
    operations.iter()
        .for_each(|o| {
            // x00 and y00 should have output z00
            if o.is_first() {
                if !o.has_output("z00".to_string()) {
                    sus.push(o.o.clone());
                }
                return;
            } else if o.has_output("z00".to_string()) {
                sus.push(o.o.clone());
            }

            if o.is_output() {
                sus.push(o.o.clone());
            }
        });
    sus
}

// Find suspicious XOR operation not related to xN and yN
// It should only have a zN as a result
fn find_suspicious_indirect_xor_operations(operations: &Vec<Operation>) -> Vec<String> {
    let mut sus = Vec::new();
    operations.iter()
        .for_each(|o| {
            if !o.is_output() {
                sus.push(o.o.clone());
            }
        });
    sus
}

// Find suspicious outputs
// It should only be the result of an XOR operation
fn find_suspicious_output(operations: &Vec<Operation>) -> Vec<String> {
    let mut sus = Vec::new();
    operations.iter()
        .for_each(|o| {
            // Hard coded value for max N value in input
            if o.has_output("z45".to_string()) {
                if !o.has_operand(Operand::OR) {
                    sus.push(o.o.clone());
                }
                return;
            } else if !o.has_operand(Operand::XOR) {
                sus.push(o.o.clone());
            }

        });
    sus
}

// Imposters hidden after the obvious checks
// Direct XOR (xN, yN) should link to one indirect XOR (with result zN)
fn find_suspicious_link(direct: &Vec<Operation>, indirect: &Vec<Operation>, sus: &mut HashSet<String>) -> Vec<Operation> {
    let mut imposters = Vec::new();

    direct.iter().for_each(
        |o| {
            // Already thrown in space or is z00, skip
            if sus.contains(&o.o) || o.has_output("z00".to_string()) {
                return;
            }

            let count = indirect.iter()
                .filter(|&io| io.has_input(o.o.clone()))
                .count();
            if count == 0  {
                imposters.push(o.clone());
                sus.insert(o.o.clone());
            }
        }
    );

    imposters
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, operations) = parse(input);

    let mut potential_gates: HashSet<String> = HashSet::new();

    let mut direct_xor_operations = Vec::new();
    let mut indirect_xor_operations = Vec::new();
    let mut output_operations = Vec::new();
    operations.iter().for_each(
        |o| {
            if o.is_direct() && o.has_operand(Operand::XOR) {
                direct_xor_operations.push(o.clone());
            } else if !o.is_direct() && o.has_operand(Operand::XOR) {
                indirect_xor_operations.push(o.clone());
            } else if o.is_output() {
                output_operations.push(o.clone());
            }
        }
    );

    // Search for wrong operations
    // The input is like a Ripple-carry adder, so for each part, identify issues inside
    potential_gates.extend(find_suspicious_direct_xor_operations(&direct_xor_operations));
    potential_gates.extend(find_suspicious_indirect_xor_operations(&indirect_xor_operations));
    potential_gates.extend(find_suspicious_output(&output_operations));

    let to_check: Vec<Operation> = find_suspicious_link(&direct_xor_operations, &indirect_xor_operations, &mut potential_gates);
    to_check.iter().for_each(
        |o| {
            let (_, rest) = o.i1.split_at(1);
            let expected_output = format!("z{}", rest);

            let matching_operations = indirect_xor_operations.iter()
                .find(|o| o.has_output(expected_output.clone()));

            if matching_operations.is_none() {
                panic!("No result for output !");
            }
            let mo = matching_operations.unwrap();

            let potential_match = operations.iter()
                .find(|op| {
                    if !op.has_operand(Operand::OR) {
                        return false;
                    }
                    return op.has_output(mo.i1.clone()) || op.has_output(mo.i2.clone());
                });

            if potential_match.is_none() {
                panic!("Not result found, reconsider the impl ;(")
            }

            if potential_match.unwrap().o.clone() == mo.i1 {
                potential_gates.insert(mo.i2.clone());
            } else {
                potential_gates.insert(mo.i1.clone());
            }
        }
    );

    let mut to_swap: Vec<String> = potential_gates.into_iter().collect();
    to_swap.sort();

    Some(to_swap.iter().join(","))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
