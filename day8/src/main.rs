use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let code = parse_code(read_in_file("src/code.txt"));

    println!("---- PART 1 ----");
    debug_code(code.clone());

    println!("\n\n---- PART 2 ----");
    correct_code(code.clone());
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_file(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()
}

/// Parse the code from a vector of strings and return a vector of Instructions
/// Each line is of the form [instr] [num]
fn parse_code(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let temp: Vec<&str> = line.split_whitespace().collect();
        let instr = String::from(temp[0]);
        let value = temp[1].parse::<i64>().unwrap();
        let line_num = i as u64;
        let instruction = Instruction{
            line_num,
            instr,
            value,
        };
        instructions.push(instruction);
    }
    instructions
}


/// A structure to store the current memory state of the program
/// 
/// # Parameters
/// 
/// * `current_line` the current line of execution
/// * `accumulator` the current value of the accumulator (before execution of current_line)
#[derive(Debug, Clone)]
struct Memory {
    current_line: u64,
    accumulator: i64,
}

/// A structure containing a code instruction
/// 
/// # Parameters
/// 
/// * `line_num` the line number of the instruction used for debug purposes
/// * `instr` a string containing the instruction
/// * `value` the number to apply to the instruction
#[derive(Debug, Clone)]
struct Instruction {
    line_num: u64,
    instr: String,
    value: i64,
}

impl PartialEq for Instruction {
    /// We consider two instructions equal if they have the same line number
    fn eq(&self, other: &Instruction) -> bool {
        self.line_num == other.line_num
    }
}

/// Run through the code and print the memory state when we reach an exception or the program end
fn debug_code(code: Vec<Instruction>) -> Memory {
    let mut memory = Memory{
        current_line: 0,
        accumulator: 0
    };

    let mut history: Vec<u64> = Vec::new();
    
    loop {
        history.push(memory.current_line);
        if let Some(instr) = code.get(memory.current_line as usize) {
            // RUN CODE
            memory = run_instruction(instr, memory);
        } else {
            // PROGRAM ENDED
            println!("END: Memory state is {:?}", memory);
            return memory;
        }

        if history.contains(&memory.current_line) {
            println!("ERROR: Infinite Loop Reached memory state is {:?}", memory);
            return memory
        }
    }  
}

/// Attempt to correct one nop or jmp instruction so the code runs to completion
/// 
/// # Parameters
/// 
/// 
fn correct_code(mut code: Vec<Instruction>) -> Memory{
    
    let mut memory = Memory{
        current_line: 0,
        accumulator: 0
    };
    
    let mut history: Vec<Instruction> = Vec::new();
    let mut memory_snapshot = None;
    let mut history_snapshot = None;
    let code_snapshot = code.clone();
    
    loop {
        let code_copy = code.clone();
        if let Some(line) = code_copy.get(memory.current_line as usize) {
            if history.contains(line) {
                // Roll back to previous snapshot if exists
                if let Some(memory_old) = memory_snapshot {
                    memory = memory_old;
                    history = history_snapshot.unwrap();
                    code = code_snapshot.clone();
                }
                // Find the next instruction we can try to modify
                loop {
                    if let Some(instr) = history.pop() {
                        // We modify the memory to roll back the state as we pop off instructions
                        match instr.instr.as_str() {
                            "jmp" => {
                                code.get_mut(instr.line_num as usize).unwrap().instr = String::from("nop");
                                memory.current_line = instr.line_num;
                                break
                            }
                            "nop" => {
                                code.get_mut(instr.line_num as usize).unwrap().instr = String::from("jmp");
                                memory.current_line = instr.line_num;
                                break;
                            }
                            "acc" => {
                                memory.accumulator -= instr.value;
                            }
                            _ => {
                                println!("ERROR: Found unsupported instruction");
                                return memory
                            }
                        }
                    } else {
                        println!("ERROR: No more lines to correct");
                        return memory
                    }
                }
                // Save snapshot
                memory_snapshot = Some(memory.clone());
                history_snapshot = Some(history.clone());
                continue;
            }
            history.push(line.clone());
            memory = run_instruction(line, memory);
        } else {
            // PROGRAM ENDED
            println!("PROGRAM END: {:?}", memory);
            return memory
        }
    }
}

/// Run an instruction. Currently accepts the following instructions:
/// * acc [value] - add value to the accumulator, goto the next line
/// * jmp [value] - jump value lines in the program
/// * nop [value] - perform no operation, goto the next line
/// 
/// # Parameters
/// 
/// * `instr` the instruction to execute
/// * `mem` the memory to manupulate
/// 
/// # Returns
/// 
/// * the new memory state
fn run_instruction(instr: &Instruction, mem: Memory) -> Memory {
    match instr.instr.as_str() {
        "acc" => Memory{ current_line: mem.current_line + 1, accumulator: mem.accumulator + instr.value},
        "jmp" => Memory{ current_line: (mem.current_line as i64 + instr.value) as u64, accumulator: mem.accumulator},
        "nop" => Memory{ current_line: mem.current_line + 1, accumulator: mem.accumulator},
        _ => Memory{ current_line: mem.current_line, accumulator: mem.accumulator},        
    }
}