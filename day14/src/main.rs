use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let lines = read_in_lines("src/code.txt");
    let code = parse_code(lines);
    let mut mask: String = String::new();

    println!("\n--- Part 1 ---\n");
    let mut memory1: HashMap<u64, u64> = HashMap::new();
    for command in code.iter() {
        match command[0].as_str() {
            "mask" => mask = String::from(command[1].as_str()),
            "mem" => {
                let address = command[1].parse::<u64>().unwrap();
                let value = command[2].parse::<u64>().unwrap();
                memory1.insert(address, apply_value_mask(value, &mask));
            }
            _ => panic!("Instruction not recognised: {:?}", command),
        }
    }
    // Sum over the whole memory
    let mut total: u64 = 0;
    for (_, value) in memory1.iter() {
        total += value;        
    }
    println!("Total value in memory is {}", total);

    println!("\n--- Part 2 ---\n");
    let mut memory2: HashMap<u64, u64> = HashMap::new();
    for command in code.iter() {
        match command[0].as_str() {
            "mask" => mask = String::from(command[1].as_str()),
            "mem" => {
                let address = command[1].parse::<u64>().unwrap();
                let value = command[2].parse::<u64>().unwrap();
                write_address_with_mask(address, value, &mask, &mut memory2);
            }
            _ => panic!("Instruction not recognised: {:?}", command),
        }
    }
    // Sum over the whole memory
    let mut total: u64 = 0;
    for (_, value) in memory2.iter() {
        total += value;        
    }
    println!("Total value in memory is {}", total);
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_lines(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()
}

/// Parse the lines of code and return each line of instructions as a Vec<String>
/// 
/// # Arguments
/// 
/// * `lines` the lines to parse
fn parse_code(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut code = Vec::new();
    
    for line in lines {
        let mut instruction: Vec<String> = Vec::new();
        let temp: Vec<&str> = line.split("=").collect();
        let lhs = temp[0].trim();
        let rhs = temp[1].trim();

        match lhs {
            "mask" => {
                instruction.push(String::from(lhs));
                instruction.push(String::from(rhs));
            }
            _ => {
                let temp2: Vec<&str> = lhs.split("[").collect();
                let instr = temp2[0];
                let mut num = String::from(temp2[1]);
                num.pop(); // remove the last charachter
                instruction.push(String::from(instr));
                instruction.push(String::from(num));
                instruction.push(String::from(rhs));
            }
        }
        code.push(instruction);
    }
    code
}

/// Apply the given bit mask to the supplied value
/// 
/// # Arguments
/// 
/// * `value` the value to modify
/// * `mask` the mask to apply
fn apply_value_mask(value: u64, mask: &String) -> u64 {
    let mut bit = 0;
    let mut result = 0;   
    for c in mask.chars().rev() {
        let mut current_bit = (value >> bit) & 1;
        match c {
            'X' => (),
            '1' => current_bit = 1,
            '0' => current_bit = 0,
            _ => panic!("An invalid charachter ({}) was found in the mask", c),
        }
        result += current_bit << bit;
        bit += 1;
    }
    result
}

/// A somewhat recursive function to write to addresses with the given address, value and mask.
/// 
/// # Arguments
/// 
/// * `address` the commanded address
/// * `value` the value to write
/// * `mask` the mask to apply
/// * `memory` the memory map to write to
fn write_address_with_mask(address: u64, value: u64, mask: &String, memory: &mut HashMap<u64, u64>) {
    // println!("write_address_with_mask called with mask {}", mask);
    let mut bit = 0;
    let mut result = 0;
    let mut head = mask.clone();
    let mut tail: String = String::new(); 
    while let Some(c) = head.pop() {
        let mut current_bit = (address >> bit) & 1;
        tail.insert(0, '0');
        match c {
            'X' => {
                let a1 = ((address >> bit) << bit) + result;  // empty the last bits
                let mut a2 = a1;
                if current_bit == 0 {
                    a2 = a2 | (1 << bit);
                } else {
                    a2 = a2 ^ (1 << bit);
                }
                write_address_with_mask(a1, value, &format!("{}{}", head, tail), memory);
                write_address_with_mask(a2, value, &format!("{}{}", head, tail), memory);
                return;
            }
            '1' => {
                current_bit = 1;
            },
            '0' => (),
            _ => panic!("An invalid charachter ({}) was found in the mask", c),
        }
        result += current_bit << bit;
        bit += 1;
    }
    // println!("write {} to {}", value, result);
    memory.insert(result, value);
}
